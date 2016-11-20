use prelude::*;

use game::GameState;
use key::KeyboardState;

use glutin::VirtualKeyCode as Key;
use glutin::ElementState::Pressed;
use glutin::Event;
use glutin::MouseButton;

#[derive(Debug)]
pub enum InternalEvent {
	/// Quits the game.
	Quit,
	/// Moves the character relative to their current viewpoint. This will be scaled based on the character's current speed.
	/// +x is right
	/// +y is up
	/// +z is back
	Move(Vec3),
	/// Rotate the player
	/// x is yaw
	/// y is pitch
	Rotate(Vector2<Rad<Flt>>),
	/// Focus on the application (enable mouse locking)
	Focus,
	/// Unfocus from the application (disable mouse locking)
	Unfocus,
	/// Reloads the meshes
	ReloadMeshes,
	/// Reloads the textures
	ReloadTextures,
	/// Reloads the shaders
	ReloadShaders,
}
impl InternalEvent {
	pub fn from_events<I>(state: &mut GameState, it: &mut I) -> Vec<InternalEvent> where I: Iterator<Item=Event> {
		let mut mouse_pos = state.mid_screen;
		let mut mouse_rel = vec2(0, 0);
		
		let mut ret = vec![];
		for e in it {
			match e {
				Event::KeyboardInput(element_state, _, Some(key)) => {
					state.keyboard_state.set_key_state(key, element_state);
					if state.focused && element_state == Pressed {
						key_pressed(&mut ret, key);
					}
				},
				Event::MouseMoved(x, y) => {
					if state.focused {
						if let Some((prev_x, prev_y)) = mouse_pos {
							trace!("Mouse Moved: Abs: {}, {}", x, y);
							let rel = vec2(x - prev_x, y - prev_y);
							mouse_pos = Some((x, y));
							mouse_rel += rel;
						}
					}
				},
				Event::MouseInput(Pressed, MouseButton::Left) => {
					ret.push(InternalEvent::Focus);
				},
				Event::Closed => {
					ret.push(InternalEvent::Quit);
				},
				_ => {}
			}
		}
		
		process_mouse_movement(&state, mouse_rel, &mut ret);
		
		process_keyboard_state(&state.keyboard_state, &mut ret);
		if state.ignore_mouse_frames > 0 {
			state.ignore_mouse_frames -= 1;
		}
		ret
	}
}

fn process_mouse_movement(state: &GameState, rel: Vector2<i32>, es: &mut Vec<InternalEvent>) {
	if rel != zero() {
		let yaw   = Rad(rel.x as Flt * 0.003);
		let pitch = Rad(rel.y as Flt * 0.003);
		if state.ignore_mouse_frames == 0 {
			es.push(InternalEvent::Rotate(vec2(yaw, pitch)));
			trace!("Mouse Moved: Rel: {:3}, {:3} | Rot: {:.3?}, {:.3?}", rel.x, rel.y, yaw, pitch);
		} else {
			trace!("MOUSE IGNORED: Rel: {:3}, {:3} | Rot: {:.3?}, {:.3?}", rel.x, rel.y, yaw, pitch);
		}
	}
}

fn process_keyboard_state(state: &KeyboardState, es: &mut Vec<InternalEvent>) {
	let spd: Flt = if state.is_key_down(Key::LShift) || state.is_key_down(Key::RShift) { 2.0 } else { 1.0 };
	
	if state.is_key_down(Key::W) { es.push(InternalEvent::Move(spd * vec3( 0.0,  0.0, -1.0))); }
	if state.is_key_down(Key::S) { es.push(InternalEvent::Move(spd * vec3( 0.0,  0.0,  1.0))); }
	if state.is_key_down(Key::A) { es.push(InternalEvent::Move(spd * vec3(-1.0,  0.0,  0.0))); }
	if state.is_key_down(Key::D) { es.push(InternalEvent::Move(spd * vec3( 1.0,  0.0,  0.0))); }
	if state.is_key_down(Key::Q) { es.push(InternalEvent::Move(spd * vec3( 0.0,  1.0,  0.0))); }
	if state.is_key_down(Key::E) { es.push(InternalEvent::Move(spd * vec3( 0.0, -1.0,  0.0))); }
}

fn key_pressed(es: &mut Vec<InternalEvent>, key: Key) {
	match key {
		Key::Escape => { es.push(InternalEvent::Unfocus); }
		Key::F5 => {
			 es.push(InternalEvent::ReloadMeshes);
			 es.push(InternalEvent::ReloadTextures);
			 es.push(InternalEvent::ReloadShaders);
		},
		Key::F6 => { es.push(InternalEvent::ReloadMeshes); },
		Key::F7 => { es.push(InternalEvent::ReloadTextures); },
		Key::F8 => { es.push(InternalEvent::ReloadShaders); },
		_ => {}
	}
}
