use prelude::*;

use glium::Frame;

use super::{Entity, Transform};
use render::{MeshID, Render};

pub struct RandomRotatingEntity {
	time: Flt,
	trans: Transform,
	mesh_id: MeshID,
}
impl RandomRotatingEntity {
	pub fn new<T: Into<Transform>>(trans: T, mesh_id: MeshID) -> RandomRotatingEntity {
		RandomRotatingEntity {
			time: 0.0,
			trans: trans.into(),
			mesh_id: mesh_id,
		}
	}
}
impl Entity for RandomRotatingEntity {
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
		let rot = rot.slerp(target, dt / 0.5);
		self.trans.set_rot(rot);
	}
}

pub struct RotatingEntity {
	axis: Vec3,
	angle: Rad<Flt>,
	trans: Transform,
	mesh_id: MeshID,
}
impl RotatingEntity {
	pub fn new<T: Into<Transform>>(trans: T, axis: Vec3, angle: Rad<Flt>, mesh_id: MeshID) -> RotatingEntity {
		RotatingEntity {
			axis: axis,
			angle: angle,
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
		let rot = self.trans.rot();
		let angle = self.angle * dt;
		let rot_trans = Quat::from_axis_angle(self.axis, angle);
		
		self.trans.set_rot(rot * rot_trans);
	}
}
