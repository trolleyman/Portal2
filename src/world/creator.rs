use prelude::*;

use render::{self, Camera};
use world::*;

/// TODO: create world
pub fn example_world() -> World {
	let cam = Camera::default();
	
	let mut es = vec![];
	es.push(Entity::new(vec3(0.0, 0.0, -5.0), render::MESHID_TEST.into()));
	es.push(Entity::new(vec3(2.0, 0.0, -5.0), render::MESHID_MONKEY.into()));
	
	World {
		camera: cam,
		entities: es,
	}
}