use prelude::*;

use render::{MESHID_TEST, Camera};
use world::*;

/// TODO: create world
pub fn example_world() -> World {
	let cam = Camera::default();
	
	let mut es = vec![];
	es.push(Entity::new(vec3(0.0, 0.0, -5.0), MESHID_TEST.into()));
	
	World {
		camera: cam,
		entities: es,
	}
}