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
	/// Location of the two portals in the world
	portals: Option<[entity::Portal; 2]>,
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
	
	pub fn rotate_portal(&mut self, r: Vector2<Rad<Flt>>) {
		self.portals.map(|mut ps| { ps[1].angx += r.x; } );
		self.portals.map(|mut ps| { ps[1].angy += r.y; } );
	}
	
	pub fn tick(&mut self, dt: Flt) {
		for e in self.entities.iter_mut() {
			e.tick(dt);
		}
	}
	
	fn render_iter(&self, r: &mut Render, f: &mut Frame) {
		for e in self.entities.iter() {
			e.render(r, f);
		}
		if let Some(ps) = self.portals {
			ps[0].render(r, f);
			ps[1].render(r, f);
		}
	}
	
	pub fn render(&self, r: &mut Render, f: &mut Frame) {
		r.set_camera(self.camera.clone());
		r.set_light(self.light);
		if let Some(ps) = self.portals {
			r.set_portals(f, ps[0], ps[1]);
			self.render_iter(r, f);
			r.set_portals(f, ps[1], ps[0]);
			self.render_iter(r, f);
		}
		r.unset_portals(f);
		self.render_iter(r, f);
	}
}
