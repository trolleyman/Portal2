#![allow(non_snake_case)]
#![feature(question_mark, type_ascription, question_mark_carrier, slice_patterns)]
extern crate cgmath as cg;
//extern crate gl;
extern crate glutin;

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
use render::Render;

pub fn main() {
	let m = render::mesh::Mesh::from_string("\
# test
       #test
v 0 0 0 #test3
v 1 0 0
v 0 1 0

f 0 1 2 # endgfiletest".into());
	
	println!("{:#?}", m);
	
	unimplemented!();
	
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