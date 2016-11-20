use prelude::*;

use super::{Entity, Transform};
use render::{MESHID_PORTAL, Render};

use glium::Frame;

/// A structure representing a portal
/// 
/// IMPORTANT: The functions in this file assume that the mesh is oriented so that
///            the portal itself is centered around [0.0, 0.0, 0.0]. The normal
///            is also assumed to be [0.0, 0.0, 1.0]. The portal size is assumed to
///            be [1.0, 1.0].
#[derive(Debug, Copy, Clone)]
pub struct Portal {
	pub pos: Vec3,
	pub angx: Rad<Flt>,
	pub angy: Rad<Flt>,
	pub size: Vec2,
}
impl Portal {
	pub fn new(pos: Vec3, angx: Rad<Flt>, angy: Rad<Flt>, size: Vec2) -> Portal {
		Portal {
			pos: pos,
			angx: angx,
			angy: angy,
			size: size,
		}
	}
	
	/// Returns the model matrix for the portal
	pub fn model_matrix(&self) -> Mat4 {
		// This is confusing because it uses from_angle_y.
		// This is because the x rotation is /around/ the y axis. 
		let rot_x = Quaternion::from_angle_y(self.angx);
		// The same principle applies here.
		let rot_y = Quaternion::from_angle_x(self.angy);
		// TODO: Check if this is the right order to rotate in
		let rot = rot_x * rot_y;
		let trans = Transform::new_rot(self.pos, rot, self.size.extend(1.0));
		trans.mat()
	}
}
impl Entity for Portal {
	fn render(&self, r: &mut Render, f: &mut Frame) {
		r.draw_mesh(f, MESHID_PORTAL.into(), self.model_matrix());
	}
	fn tick(&mut self, _dt: Flt) {}
}