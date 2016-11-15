use prelude::*;

pub mod creator;
pub mod entity;

pub use self::entity::{Transform, Entity};

use glium::Frame;

use render::{Camera, Light, Render};

#[allow(dead_code)]
pub struct World {
	/// Main camera in the world
	camera: Camera,
	/// Light source in the world
	light: Light,
	/// Entities in the world. All are static atm.
	entities: Vec<Box<Entity>>,
}
impl World {
	pub fn new() -> GameResult<World> {
		let w = creator::example_world();
		Ok(w)
	}
	
	pub fn camera(&self) -> &Camera {
		&self.camera
	}
	
	pub fn light(&self) -> &Light {
		&self.light
	}
	
	pub fn move_player(&mut self, v: Vec3) {
		self.camera.move_camera(v);
	}
	
	pub fn rotate_player(&mut self, r: Vector2<Rad<Flt>>) {
		self.camera.rotate_player(r);
	}
	
	pub fn tick(&mut self, dt: Flt) {
		for e in self.entities.iter_mut() {
			e.tick(dt);
		}
	}
	
	pub fn render(&self, r: &mut Render, f: &mut Frame) {
		r.set_camera(self.camera.clone());
		r.set_light(self.light);
		for e in self.entities.iter() {
			e.render(r, f);
		}
	}
}
