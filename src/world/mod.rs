use prelude::*;

pub mod creator;
mod entity;

pub use self::entity::{Transform, Entity};

use glium::Frame;

use render::{Camera, Render};

#[allow(dead_code)]
pub struct World {
	/// Main camera in the world
	camera: Camera,
	/// Entities in the world. All are static atm.
	entities: Vec<Entity>,
}
impl World {
	pub fn new() -> GameResult<World> {
		let w = creator::example_world();
		Ok(w)
	}
	
	pub fn camera(&self) -> &Camera {
		&self.camera
	}
	
	pub fn move_player(&mut self, v: Vec3) {
		self.camera.move_camera(v);
	}
	
	pub fn render(&self, r: &mut Render, f: &mut Frame) {
		r.set_camera(self.camera.clone());
		for e in self.entities.iter() {
			e.render(r, f);
		}
	}
}
