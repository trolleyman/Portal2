use prelude::*;

use render::mesh::MeshID;

pub struct Entity {
	pos: Vec3,
	mesh: MeshID,
}
impl Entity {
	pub fn new(pos: Vec3, mesh: MeshID) -> Entity {
		Entity::new_ext(pos, zero(), one(), mesh)
	}
	
	pub fn new_ext(pos: Vec3, _scale: Vec3, _rot: Quat, mesh: MeshID) -> Entity {
		Entity {
			pos: pos,
			// TODO: Calculate rotation & scaling
			mesh: mesh,
		}
	}
}