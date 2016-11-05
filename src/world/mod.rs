use prelude::*;

pub mod creator;
pub mod entity;

use world::entity::Entity;

pub struct World {
	/// Entities in the world. All are static atm.
	entities: Vec<Entity>,
	/// Player's position in the world
	player_pos: Vec3,
	/// Player's x-angle (looking to the right)
	player_angx: Rad<Flt>,
	/// Player's y-angle (looking upwards)
	player_angy: Rad<Flt>,
}
impl World {
	pub fn new() -> GameResult<World> {
		let w = creator::example_world();
		Ok(w)
	}
	
	pub fn move_player(&mut self, v: Vec3) {
		self.player_pos += v;
	}
}
