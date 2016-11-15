use prelude::*;

use glium::Frame;

use super::{Entity, Transform};
use render::{MeshID, Render};

pub struct RotatingEntity {
	time: Flt,
	trans: Transform,
	mesh_id: MeshID,
}
impl RotatingEntity {
	pub fn new<T: Into<Transform>>(trans: T, mesh_id: MeshID) -> RotatingEntity {
		RotatingEntity {
			time: 0.0,
			trans: trans.into(),
			mesh_id: mesh_id,
		}
	}
}
impl Entity for RotatingEntity {
	fn render(&self, r: &mut Render, f: &mut Frame) {
		r.draw_mesh(f, self.mesh_id.clone(), self.trans.mat());
	}
	fn tick(&mut self, dt: Flt) {
		self.time += dt;
		
		let rot = self.trans.rot();
		let x = (self.time / 1.51 - 65.124).cos();
		let y = (self.time * 1.44 - 12.145).sin();
		let z = (self.time * 1.14 - 41.624).cos();
		let axis = vec3(x, y, z).normalize();
		
		let target = Quaternion::from_axis_angle(axis, Rad(self.time));
		let rot = rot.slerp(target, 1.0);
		self.trans.set_rot(rot);
	}
}
