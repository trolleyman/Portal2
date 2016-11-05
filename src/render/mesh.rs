use prelude::*;

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::rc::Rc;

use glium::backend::Context;
use glium::VertexBuffer;

/// Meshes are identified by their filename
pub type MeshID = String;

pub const MESHID_TEST: &'static str = "res/meshes/test.obj";

pub struct MeshBank {
	ctx: Rc<Context>,
	cache: HashMap<MeshID, Mesh>,
}
impl MeshBank {
	pub fn new(ctx: Rc<Context>) -> MeshBank {
		MeshBank {
			ctx: ctx,
			cache: HashMap::new(),
		}
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
	pub fn from_file(ctx: &Rc<Context>, filename: &str) -> GameResult<Mesh> {
		unimplemented!()
	}
}
