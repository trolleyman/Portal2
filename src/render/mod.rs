use prelude::*;

use self::mesh::MeshBank;

pub mod mesh;

pub struct Render {
	mesh_bank: MeshBank,
}
impl Render {
	pub fn new() -> Result<Render, String> {
		Ok(Render {
			mesh_bank: MeshBank::new(),
		})
	}
}
