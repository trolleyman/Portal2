#![allow(non_snake_case)]
#![feature(question_mark, type_ascription, question_mark_carrier)]
extern crate cgmath as cg;
//extern crate gl;
extern crate glutin;

mod prelude;
mod event;
mod entity;
mod game;
mod mesh;
mod render;
mod result;
mod world;

use prelude::*;

use std::io::{self, Write};
use std::process::exit;

use game::Game;
use render::Render;

pub fn main() {
	match run().into() {
		Err(e) => {
			writeln!(io::stderr(), "error: {}", e);
			exit(1);
		},
		_ => {}
	}
}

pub fn run() -> GameResult<()> {
	let mut game = Game::new()?;
	game.run()
}