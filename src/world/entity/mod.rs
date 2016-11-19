use prelude::*;

use glium::Frame;

use render::Render;

pub use self::simple::SimpleEntity;
pub use self::rotating::{RotatingEntity, RandomRotatingEntity};

mod simple;
mod rotating;

#[derive(Debug, Copy, Clone)]
pub struct Transform {
	pos: Vec3,
	rot: Quat,
	scale: Vec3,
	mat: Mat4,
}
impl Default for Transform {
	fn default() -> Transform {
		Transform::from_pos(zero())
	}
}
impl Transform {
	pub fn from_pos(pos: Vec3) -> Transform {
		Transform::new(pos, Vec3::from_value(1.0))
	}
	pub fn new(pos: Vec3, scale: Vec3) -> Transform {
		Transform::new_rot(pos, one(), scale)
	}
	pub fn new_rot(pos: Vec3, rot: Quat, scale: Vec3) -> Transform {
		let mut t = Transform {
			pos: pos,
			rot: rot,
			scale: scale,
			mat: one(),
		};
		t.recalc_mat();
		t
	}
	
	fn recalc_mat(&mut self) {
		self.mat = Mat4::from_translation(self.pos)
		         * Mat4::from(self.rot)
		         * Mat4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);
	}
	pub fn mat(&self) -> Mat4 {
		self.mat
	}
	
	pub fn pos(&self) -> Vec3 {
		self.pos
	}
	pub fn rot(&self) -> Quat {
		self.rot
	}
	pub fn scale(&self) -> Vec3 {
		self.scale
	}
	pub fn set_pos(&mut self, pos: Vec3) {
		self.pos = pos;
		self.recalc_mat();
	}
	pub fn set_rot(&mut self, rot: Quat) {
		self.rot = rot;
		self.recalc_mat();
	}
	pub fn set_scale(&mut self, scale: Vec3) {
		self.scale = scale;
		self.recalc_mat();
	}
}
impl From<Vec3> for Transform {
	fn from(pos: Vec3) -> Transform {
		Transform::from_pos(pos)
	}
}

pub trait Entity {
	/// Renders the entity to a frame
	fn render(&self, r: &mut Render, f: &mut Frame);
	/// Updates the entity every frame
	fn tick(&mut self, dt: Flt);
}
