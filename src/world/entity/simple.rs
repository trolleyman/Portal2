use prelude::*;

use glium::Frame;

use super::{Entity, Transform};
use render::{MeshID, Render};

pub struct SimpleEntity {
	trans: Transform,
	mesh_id: MeshID,
}
impl SimpleEntity {
	pub fn new<T: Into<Transform>>(trans: T, mesh_id: MeshID) -> SimpleEntity {
		SimpleEntity {
			trans: trans.into(),
			mesh_id: mesh_id,
		}
	}
}
impl Entity for SimpleEntity {
	fn render(&self, r: &mut Render, f: &mut Frame) {
		r.draw_mesh(f, self.mesh_id.clone(), self.trans.mat());
	}
	fn tick(&mut self, _dt: Flt) {
		
	}
}
