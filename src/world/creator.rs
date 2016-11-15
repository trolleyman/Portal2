use prelude::*;

use render::{self, Camera, Light};
use world::*;

pub fn example_world() -> World {
	let mut cam = Camera::default();
	cam.pos = vec3(0.0, 0.6, 0.0);
	let mut light = Light::default();
	light.pos = vec3(3.0, 10.0, 0.0);
	
	let mut es: Vec<Box<Entity>> = vec![];
	es.push(box entity::SimpleEntity::new(vec3(0.0, 0.0, -5.0), render::MESHID_TEST.into()));
	es.push(box entity::RandomRotatingEntity::new(vec3(3.0, 0.0, -5.0), render::MESHID_MONKEY.into()));
	es.push(box entity::RotatingEntity::new(vec3(-5.0, -1.0, -5.0), vec3(0.0, 1.0, 0.0), Rad(1.0), render::MESHID_TEAPOT.into()));
	es.push(box entity::SimpleEntity::new(vec3(0.0, 0.0, 0.0), render::MESHID_FLOOR.into()));
	es.push(box entity::RotatingEntity::new(vec3(0.0, 3.0, -5.0), vec3(0.0, 1.0, 0.0), Rad(0.5), render::MESHID_EARTH.into()));
	
	World {
		camera: cam,
		light: light,
		entities: es,
	}
}