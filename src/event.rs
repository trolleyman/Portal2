use prelude::*;

use glutin::PollEventsIterator;
use glutin::VirtualKeyCode as Key;
use glutin::ElementState::Pressed;
use glutin::Event;
use glutin::MouseButton;

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
	pub fn from_events(it: &mut PollEventsIterator) -> Vec<InternalEvent> {
		let mut ret = vec![];
		for e in it {
			match e {
				Event::KeyboardInput(Pressed, _, Some(key)) => {
					key_pressed(&mut ret, key);
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
		ret
	}
}

fn key_pressed(es: &mut Vec<InternalEvent>, key: Key) {
	match key {
		Key::Escape => { es.push(InternalEvent::Unfocus); }
		Key::W => { es.push(InternalEvent::Move(vec3( 0.0,  0.0, -1.0))); },
		Key::S => { es.push(InternalEvent::Move(vec3( 0.0,  0.0,  1.0))); },
		Key::A => { es.push(InternalEvent::Move(vec3( 1.0,  0.0,  0.0))); },
		Key::D => { es.push(InternalEvent::Move(vec3(-1.0,  0.0,  0.0))); },
		Key::Q => { es.push(InternalEvent::Move(vec3( 0.0,  1.0,  0.0))); },
		Key::E => { es.push(InternalEvent::Move(vec3( 0.0, -1.0,  0.0))); },
		_ => {}
	}
}
