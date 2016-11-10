use prelude::*;

use std::collections::HashMap;
use std::rc::Rc;

use glium::backend::Context;
use glium::VertexBuffer;

/// Meshes are identified by their filename
pub type MeshID = String;

pub const MESH_DIR: &'static str = "res/mesh/";
pub const MESHID_TEST: &'static str = "res/mesh/test.obj";

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
		// If cache doesn't exist, loads it from a file.
		if self.cache.get(&id).is_none() {
			self.cache.insert(id.clone(), Mesh::from_file(&self.ctx, &id)?);
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

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
	pos: [Flt; 3],
	normal: [Flt; 3],
	uv: [Flt; 2],
}
implement_vertex!(Vertex, pos, normal, uv);

#[derive(Debug)]
pub struct Mesh {
	vertices: VertexBuffer<Vertex>,
}
impl Mesh {
	pub fn from_file(_ctx: &Rc<Context>, rel_path: &str) -> GameResult<Mesh> {
		use render::parse::ObjFile;
		
		let _f = ObjFile::new(rel_path.to_string())
			.map_err(|e| format!("Invalid mesh: {}", e));
		unimplemented!()
	}
}
