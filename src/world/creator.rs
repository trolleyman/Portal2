use prelude::*;

use render::{self, Camera, Light};
use world::*;

pub fn example_world() -> World {
	let cam = Camera::default();
	let mut light = Light::default();
	light.pos = vec3(3.0, 5.0, 0.0);
	
	let mut es: Vec<Box<Entity>> = vec![];
	es.push(box entity::SimpleEntity  ::new(vec3(0.0, 0.0, -5.0), render::MESHID_TEST.into()));
	es.push(box entity::RotatingEntity::new(vec3(3.0, 0.0, -5.0), render::MESHID_MONKEY.into()));
	
	World {
		camera: cam,
		light: light,
		entities: es,
	}
}