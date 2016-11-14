#[allow(unused_imports)]
use prelude::*;

use std::collections::HashSet;

use glutin::VirtualKeyCode as Key;
use glutin::ElementState;

pub struct KeyboardState {
	pressed_keys: HashSet<Key>,
}
impl KeyboardState {
	pub fn new() -> KeyboardState {
		KeyboardState {
			pressed_keys: HashSet::new(),
		}
	}
	pub fn key_state(&self, key: Key) -> ElementState {
		if self.pressed_keys.contains(&key) {
			ElementState::Pressed
		} else {
			ElementState::Released
		}
	}
	pub fn is_key_down(&self, key: Key) -> bool {
		self.key_state(key) == ElementState::Pressed
	}
	pub fn is_key_up(&self, key: Key) -> bool {
		self.key_state(key) == ElementState::Released
	}
	
	pub fn set_key_state(&mut self, key: Key, state: ElementState) {
		match state {
			ElementState::Pressed  => { self.pressed_keys.insert(key); },
			ElementState::Released => { self.pressed_keys.remove(&key); }
		}
	}
}