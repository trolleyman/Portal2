use prelude::*;

use render::mesh;
use world::*;
use world::entity::Entity;

/// TODO: create world
pub fn example_world() -> World {
	let mut es = vec![];
	
	es.push(Entity::new(vec3(0.0, 0.0, -5.0), mesh::MESHID_TEST.into()));
	
	World {
		player_pos: zero(),
		player_angx: zero(),
		player_angy: zero(),
		entities: es,
	}
}