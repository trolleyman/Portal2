use prelude::*;

use render::{self, Camera, Light};
use world::*;

pub fn example_world() -> World {
	let mut cam = Camera::default();
	cam.pos = vec3(0.0, 1.6, 0.0);
	let mut light = Light::default();
	light.ambient = vec4(0.01, 0.01, 0.01, 1.0);
	light.pos = vec3(3.0, 10.0, 0.0);
	
	let mut es: Vec<Box<Entity>> = vec![];
	es.push(box entity::SimpleEntity::new(
		Transform::new(vec3(-2.0, 0.5, -4.0), Vec3::from_value(0.5)),
		render::MESHID_AXES_TEST.into()));
	es.push(box entity::RandomRotatingEntity::new(
		Transform::new(vec3(2.0, 0.7, -7.0), Vec3::from_value(0.5)),
		render::MESHID_MONKEY.into()));
	es.push(box entity::RotatingEntity::new(
		Transform::new(vec3(-3.0, -0.1, -7.0), Vec3::from_value(0.5)),
		vec3(0.0, 1.0, 0.0), Rad(1.0), render::MESHID_TEAPOT.into()));
	es.push(box entity::SimpleEntity::new(
		vec3(0.0, 0.0, 0.0),
		render::MESHID_FLOOR.into()));
	es.push(box entity::RotatingEntity::new(
		Transform::new_rot(vec3(4.0, 8.0, -8.0), Quat::from_axis_angle(Vec3::unit_x(), Rad::turn_div_2()), Vec3::from_value(4.0)),
		vec3(0.0, 1.0, 0.0), Rad(0.2), render::MESHID_EARTH.into()));
	
	let p1 = entity::Portal::new(vec3(0.0, 1.0, -7.0), vec3(0.0, 0.0, 1.0), vec2(1.0, 2.0));
	let p2 = entity::Portal::new(vec3(2.0, 1.0, -5.0), vec3(-1.0, 0.0, 0.0), vec2(1.0, 2.0));
	
	World {
		camera: cam,
		light: light,
		entities: es,
		portals: Some([p1, p2]),
	}
}