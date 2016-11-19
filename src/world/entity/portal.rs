use prelude::*;

use super::Entity;
use render::MESHID_PORTAL;

pub struct Portal {
	pub pos: Vec3,
	pub normal: Vec3,
	pub size: Vec2,
}
impl Entity for Portal {
	fn render(&self, r: &mut Render, f: &mut Frame) {
		r.draw_mesh(f, MESHID_PORTAL.into(), );
	}
	fn tick(&mut self, dt: Flt) {
		
	}
}