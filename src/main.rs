#![allow(non_snake_case)]
#![feature(question_mark, type_ascription, slice_patterns)]
extern crate cgmath as cg;
extern crate glium;
pub use glium::glutin as glutin;

pub mod prelude;
pub mod event;
pub mod game;
pub mod render;
pub mod result;
pub mod world;

use prelude::*;

use std::io::{self, Write};
use std::process::exit;

use game::Game;

pub fn main() {
	match run().into() {
		Err(e) => {
			writeln!(io::stderr(), "error: {}", e).ok();
			exit(1);
		},
		_ => {}
	}
}

pub fn run() -> GameResult<()> {
	let mut game = Game::new()?;
	game.run()
}