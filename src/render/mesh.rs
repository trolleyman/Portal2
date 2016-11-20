use prelude::*;

use std::collections::HashMap;
use std::rc::Rc;
use std::fmt;
use std::hash::{Hash, Hasher};

use glium::backend::Context;
use glium::VertexBuffer;
use glium::index::{PrimitiveType, IndicesSource, IndexBuffer, IndexBufferAny};

use game::duration_to_millis;
use super::Material;
use super::normalize_id;

/// Meshes are identified by their filename
pub type MeshID = String;

pub const MESH_DIR: &'static str = "res/mesh/";
pub const MESHID_AXES_TEST: &'static str = "res/mesh/axes_test.obj";
pub const MESHID_MONKEY: &'static str = "res/mesh/monkey.obj";
pub const MESHID_TEAPOT: &'static str = "res/mesh/teapot.obj";
pub const MESHID_FLOOR: &'static str = "res/mesh/floor.obj";
pub const MESHID_EARTH: &'static str = "res/mesh/earth.obj";
pub const MESHID_PORTAL: &'static str = "res/mesh/portal.obj";

pub struct MeshBank {
	ctx: Rc<Context>,
	cache: HashMap<MeshID, GameResult<Rc<Mesh>>>,
	default_mesh: Rc<Mesh>,
}
impl MeshBank {
	pub fn new(ctx: Rc<Context>) -> GameResult<MeshBank> {
		let buffer = VertexBuffer::new(&ctx, &vec![])
			.map_err(|e| format!("Could not initialize MeshBank: OpenGL buffer creation error: {}", e))?;
		
		let def = Mesh {
			material: Material::default(),
			vertices: buffer,
			indices: None,
		};
		
		let mut mb = MeshBank {
			ctx: ctx,
			cache: HashMap::new(),
			default_mesh: Rc::new(def),
		};
		mb.load_meshes();
		Ok(mb)
	}
	
	/// Clears the mesh cache
	pub fn clear_cache(&mut self) {
		self.cache.clear();
	}
	
	/// The default mesh
	pub fn default_mesh(&self) -> Rc<Mesh> {
		self.default_mesh.clone()
	}
	
	/// Gets a mesh from the MeshBank.
	/// 
	/// If there was an error, returns a default mesh (No vertices)
	pub fn get_mesh_or_default(&mut self, id: MeshID) -> Rc<Mesh> {
		self.get_mesh(id.clone())
			.unwrap_or(self.default_mesh())
	}
	
	/// Gets a mesh from the MeshBank
	pub fn get_mesh(&mut self, id: MeshID) -> GameResult<Rc<Mesh>> {
		// Normalize id first
		let id = normalize_id(id);
		// If cache doesn't exist, loads it from a file.
		if self.cache.get(&id).is_none() {
			use std::time::Instant;
			let t_start = Instant::now();
			let res = match Mesh::from_file(&self.ctx, &id).map(|t| Rc::new(t)) {
				Ok(t) => {
					info!("Loaded mesh: {} ({}ms)", &id, duration_to_millis(t_start.elapsed()));
					Ok(t)
				},
				Err(e) => {
					warn!("Could not load mesh ({}): {}", &id, &e);
					Err(e)
				}
			};
			self.cache.insert(id.clone(), res);
		}
		self.cache.get(&id).unwrap().clone()
	}
	
	/// Loads a mesh into the MeshBank
	pub fn load_mesh(&mut self, id: MeshID) -> GameResult<()> {
		self.get_mesh(id).map(|_| ())
	}
	
	/// Loads all of the meshes in the MESH_DIR directory
	pub fn load_meshes(&mut self) {
		use std::fs;
		use vfs;
		
		// Iterate over files in MESH_DIR
		let dir = vfs::canonicalize_exe(MESH_DIR);
		let it = match fs::read_dir(&dir) {
			Ok(it) => it,
			Err(e) => {
				warn!("Could not iterate over meshes directory ({}): {}", dir.display(), e);
				return;
			}
		};
		
		// TODO: Clean up this code - ugly but works for now
		for file in it {
			match file {
				Ok(f) => {
					let id = MESH_DIR.to_string() + &f.file_name().to_string_lossy().into_owned();
					if !id.ends_with(".obj") {
						continue;
					}
					self.load_mesh(id.clone()).ok();
				},
				_ => {} // Ignore files that return an error when iterating over them
			}
		}
	}
}

#[derive(Copy, Clone)]
pub struct Vertex {
	pos: [Flt; 3],
	uv: [Flt; 2],
	normal: [Flt; 3],
}
impl Vertex {
	pub fn as_bytes(&self) -> &[u8] {
		unsafe {
			use std::mem;
			use std::slice;
			
			let ptr = mem::transmute::<&Vertex, *const u8>(self);
			let s = slice::from_raw_parts(ptr, mem::size_of::<Vertex>());
			s
		}
	}
}
implement_vertex!(Vertex, pos, normal, uv);
impl fmt::Debug for Vertex {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		let alt = f.alternate();
		let spacing = if alt { "\n    " } else { " " };
		f.write_str("Vertex {")?;
		f.write_str(spacing)?;
		write!(f, "pos: {:?},", self.pos)?;
		f.write_str(spacing)?;
		write!(f, "uv: {:?},", self.uv)?;
		f.write_str(spacing)?;
		write!(f, "normal: {:?}", self.normal)?;
		f.write_str(if alt { "\n" } else { " " })?;
		f.write_str("}")?;
		Ok(())
	}
}
impl Hash for Vertex {
	fn hash<H>(&self, state: &mut H)
			where H: Hasher {
		// Write out everything as an array of bytes
		state.write(self.as_bytes());
	}
	fn hash_slice<H>(data: &[Self], state: &mut H)
			where H: Hasher {
		for d in data.iter() {
			d.hash(state);
		}
	}
}
impl PartialEq<Vertex> for Vertex {
	fn eq(&self, rhs: &Vertex) -> bool {
		self.as_bytes() == rhs.as_bytes()
	}
}
impl Eq for Vertex {}

pub struct Mesh {
	pub material: Material,
	pub vertices: VertexBuffer<Vertex>,
	/// If None, use NoIndices.
	pub indices: Option<Box<IndexBufferAny>>,
}
impl Mesh {
	pub fn indices_source<'a>(&'a self) -> IndicesSource<'a> {
		match &self.indices {
			&Some(ref buf) => IndicesSource::from(&**buf),
			&None => IndicesSource::NoIndices{ primitives: PrimitiveType::TrianglesList },
		}
	}
	
	pub fn from_file(ctx: &Rc<Context>, rel_path: &str) -> GameResult<Mesh> {
		use render::parse::ObjFile;
		
		let file = ObjFile::new(rel_path.to_string())
			.map_err(|e| format!("Invalid mesh: {}", e))?;
		
		// Get material
		let material = file.material.clone()
		.and_then(|mat_name| file.materials.get(&mat_name).map(Material::clone))
		.unwrap_or_else(Material::default);
		
		let mut vertices = vec![];
		let mut vertices_map: HashMap<Vertex, u32> = HashMap::new();
		let mut indices: Vec<u32> = vec![];
		
		// Change from indices to vertices
		for face in file.faces.iter() {
			for vertex in [face.x, face.y, face.z].into_iter() {
				let v = Vertex {
					pos: array3(file.vertices[vertex.vert as usize]),
					uv: array2(file.uvs[vertex.uv as usize]),
					normal: array3(file.normals[vertex.norm as usize]),
				};
				if let Some(i) = vertices_map.get(&v).cloned() {
					indices.push(i); // Use cached vertex
				} else {
					// Insert new vertex, and update the map
					let i = vertices.len() as u32;
					vertices.push(v);
					indices.push(i);
					vertices_map.insert(v, i);
				}
			}
		}
		debug!("{} vertices, {} tris loaded.", vertices.len(), indices.len() / 3);
		//trace!("Vertices loaded: {:#?}", &vertices);
		//trace!("Indices loaded: {:?}", &indices);
		
		// Upload vertex information to OpenGL
		let v_buffer = VertexBuffer::new(ctx, &vertices)
			.map_err(|e| format!("Invalid mesh ({}): OpenGL buffer creation error: {}", rel_path, e))?;
		
		// Upload index information to OpenGL
		// Minimize the size of the index array by choosing shorter ints
		let i_buffer = if vertices.len() < u8::max_value() as usize {
			// Use u8 indices
			debug!("u8 indices used.");
			let indices: Vec<_> = indices.iter().map(|&i| i as u8 ).collect();
			let buf = IndexBuffer::new(ctx, PrimitiveType::TrianglesList, &indices)
				.map_err(|e| format!("Invalid mesh ({}): OpenGL buffer creation error: {}", rel_path, e))?;
			IndexBufferAny::from(buf)
		} else if vertices.len() < u16::max_value() as usize {
			// Use u16 indices
			debug!("u16 indices used.");
			let indices: Vec<_> = indices.iter().map(|&i| i as u16).collect();
			let buf = IndexBuffer::new(ctx, PrimitiveType::TrianglesList, &indices)
				.map_err(|e| format!("Invalid mesh ({}): OpenGL buffer creation error: {}", rel_path, e))?;
			IndexBufferAny::from(buf)
		} else {
			// Use u32 indices
			debug!("u32 indices used.");
			let buf = IndexBuffer::new(ctx, PrimitiveType::TrianglesList, &indices)
				.map_err(|e| format!("Invalid mesh ({}): OpenGL buffer creation error: {}", rel_path, e))?;
			IndexBufferAny::from(buf)
		};
		
		trace!("Material loaded: {:?}", &material);
		
		Ok(Mesh {
			material: material,
			vertices: v_buffer,
			indices: Some(box i_buffer),
		})
	}
}
