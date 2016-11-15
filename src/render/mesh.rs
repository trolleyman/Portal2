use prelude::*;

use std::collections::HashMap;
use std::rc::Rc;
use std::fmt;

use glium::backend::Context;
use glium::VertexBuffer;

use super::Material;
use super::normalize_id;

/// Meshes are identified by their filename
pub type MeshID = String;

pub const MESH_DIR: &'static str = "res/mesh/";
pub const MESHID_TEST: &'static str = "res/mesh/test.obj";
pub const MESHID_MONKEY: &'static str = "res/mesh/monkey.obj";

pub struct MeshBank {
	ctx: Rc<Context>,
	cache: HashMap<MeshID, Mesh>,
}
impl MeshBank {
	pub fn new(ctx: Rc<Context>) -> MeshBank {
		let mut mb = MeshBank {
			ctx: ctx,
			cache: HashMap::new(),
		};
		mb.load_meshes();
		mb
	}
		
	/// Gets a mesh from the MeshBank
	pub fn get_mesh<'a>(&'a mut self, id: MeshID) -> GameResult<&'a Mesh> {
		// Normalize id first
		let id = normalize_id(id);
		// If cache doesn't exist, loads it from a file.
		if self.cache.get(&id).is_none() {
			self.cache.insert(id.clone(), Mesh::from_file(&self.ctx, &id)?);
			info!("Loaded mesh: {}", &id);
		}
		Ok(self.cache.get(&id).unwrap())
	}
	
	/// Loads a mesh into the MeshBank
	pub fn load_mesh(&mut self, id: MeshID) -> GameResult<()> {
		self.get_mesh(id).map(|_| ())
	}
	
	/// Loads all of the meshes in the MESH_DIR directory
	fn load_meshes(&mut self) {
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
					match self.load_mesh(id.clone()) {
						Err(e) => warn!("Could not load mesh ({}): {}", id, e),
						_ => {}
					}
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

#[derive(Debug)]
pub struct Mesh {
	pub material: Material,
	pub vertices: VertexBuffer<Vertex>,
}
impl Mesh {
	pub fn from_file(ctx: &Rc<Context>, rel_path: &str) -> GameResult<Mesh> {
		use render::parse::ObjFile;
		
		let file = ObjFile::new(rel_path.to_string())
			.map_err(|e| format!("Invalid mesh: {}", e))?;
		
		let mut vertices = Vec::with_capacity(file.faces.len() * 3);
		
		// Change from indices to vertices
		for face in file.faces.iter() {
			for vertex in [face.x, face.y, face.z].into_iter() {
				let v = Vertex {
					pos: array3(file.vertices[vertex.x as usize]),
					uv: array2(file.uvs[vertex.y as usize]),
					normal: array3(file.normals[vertex.z as usize]),
				};
				vertices.push(v);
			}
		}
		
		trace!("Vertices loaded: {:#?}", &vertices);
		
		// Upload vertex information to OpenGL
		let buffer = VertexBuffer::new(ctx, &vertices)
			.map_err(|e| format!("Invalid mesh ({}): OpenGL buffer creation error: {}", rel_path, e))?;
		
		let material = file.material.clone()
			.and_then(|mat_name| file.materials.get(&mat_name).map(Material::clone))
			.unwrap_or_else(Material::default);
		
		trace!("Material loaded: {:?}", &material);
		
		Ok(Mesh {
			material: material,
			vertices: buffer,
		})
	}
}
