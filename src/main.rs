#![allow(non_snake_case)]
#![feature(question_mark, type_ascription, slice_patterns)]
extern crate cgmath as cg;
#[macro_use]
extern crate glium;
pub use glium::glutin as glutin;
extern crate png;
#[macro_use]
extern crate log;
extern crate simplelog;

pub mod prelude;
pub mod event;
pub mod game;
pub mod render;
pub mod result;
pub mod vfs;
pub mod world;

use prelude::*;

use std::io::{self, Write};
use std::process::exit;

use game::Game;

pub fn main() {
	simplelog::SimpleLogger::init(simplelog::LogLevelFilter::Info)
		.map_err(|e| writeln!(io::stderr(), "error: logger could not be initialized: {}", e).ok())
		.ok();
	info!("logger initialized.");
	
	match run().into() {
		Err(e) => {
			error!("{}", e);
			exit(1);
		},
		_ => {}
	}
}

pub fn run() -> GameResult<()> {
	let mut game = Game::new()?;
	game.run()
}