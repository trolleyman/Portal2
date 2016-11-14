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
	/// Focus on the application (enable mouse locking)
	Focus,
	/// Unfocus from the application (disable mouse locking)
	Unfocus,
}
impl InternalEvent {
	pub fn from_events<I>(state: &mut GameState, it: &mut I) -> Vec<InternalEvent> where I: Iterator<Item=Event> {
		let mut ret = vec![];
		for e in it {
			match e {
				Event::KeyboardInput(element_state, _, Some(key)) => {
					state.keyboard_state.set_key_state(key, element_state);
					if state.focused && element_state == Pressed {
						key_pressed(&mut ret, key);
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
		process_keyboard_state(&state.keyboard_state, &mut ret);
		ret
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
		_ => {}
	}
}
