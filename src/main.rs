#![allow(non_snake_case)]
#![feature(type_ascription, slice_patterns)]
extern crate cgmath as cg;
#[macro_use]
extern crate glium;
pub use glium::glutin as glutin;
extern crate png;
#[macro_use]
extern crate log;
extern crate simplelog;

pub mod prelude;
pub mod key;
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

pub fn parse_log_level() -> Option<simplelog::LogLevelFilter> {
	use std::ascii::AsciiExt;

	let var = std::env::var("PORTAL_LOG").unwrap_or(String::new());
	let var = var.trim();
	     if var.eq_ignore_ascii_case("off"  ) { Some(simplelog::LogLevelFilter::Off  ) }
	else if var.eq_ignore_ascii_case("error") { Some(simplelog::LogLevelFilter::Error) }
	else if var.eq_ignore_ascii_case("warn" ) { Some(simplelog::LogLevelFilter::Warn ) }
	else if var.eq_ignore_ascii_case("info" ) { Some(simplelog::LogLevelFilter::Info ) }
	else if var.eq_ignore_ascii_case("debug") { Some(simplelog::LogLevelFilter::Debug) }
	else if var.eq_ignore_ascii_case("trace") { Some(simplelog::LogLevelFilter::Trace) }
	else { None }
}

pub fn main() {
	let config = simplelog::Config {
		time: Some(simplelog::LogLevel::Error),
		level: Some(simplelog::LogLevel::Error),
		target: Some(simplelog::LogLevel::Error),
		location: Some(simplelog::LogLevel::Debug),
	};

	simplelog::TermLogger::init(parse_log_level().unwrap_or(simplelog::LogLevelFilter::Info), config)
		.map_err(|e| writeln!(io::stderr(), "Error: Logger could not be initialized: {}", e).ok())
		.ok();
	info!("Logger initialized.");

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
