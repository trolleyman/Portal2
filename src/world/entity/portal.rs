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
	pub normal: Vec3,
	pub size: Vec2,
}
impl Portal {
	pub fn new(pos: Vec3, normal: Vec3, size: Vec2) -> Portal {
		Portal {
			pos: pos,
			normal: normal,
			size: size,
		}
	}
}
impl Entity for Portal {
	fn render(&self, r: &mut Render, f: &mut Frame) {
		// n = original normal in loaded mesh
		let n = vec3(0.0, 0.0, 1.0);
		let rot = Quaternion::between_vectors(n, self.normal);
		let trans = Transform::new_rot(self.pos, rot, self.size.extend(1.0));
		r.draw_mesh(f, MESHID_PORTAL.into(), trans.mat());
	}
	fn tick(&mut self, _dt: Flt) {}
}