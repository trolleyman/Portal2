use prelude::*;

use std::io::prelude::*;
use std::io;

use glutin::{Window, WindowBuilder};
use glium::backend::{Backend, Facade};
use glium::Display;
use glium::DisplayBuild;

use event::InternalEvent;
use render::Render;
use world::World;

pub struct GameState {
	/// Should the game quit?
	quit: bool,
	/// Is the game focused on?
	/// If yes, then the cursor should be reset to the middle of the window each loop.
	pub focused: bool,
}
impl Default for GameState {
	fn default() -> GameState {
		GameState {
			quit: false,
			focused: false,
		}
	}
}

pub struct Game {
	win: Display,
	ren: Render,
	world: World,
	state: GameState,
}
impl Game {
	pub fn new() -> GameResult<Game> {
		// Initialize the window
		let win = WindowBuilder::new()
			.with_title("Portal")
			.with_dimensions(800, 600)
			.with_vsync()
			.with_visibility(false)
			.build_glium()
			.map_err(|e| format!("Window creation error: {}", e))?;
				
		// Make the renderer
		let ren = Render::new(win.get_context().clone())?;
		// And the world
		let world = World::new()?;
		// And the GameState
		let state = GameState::default();
		
		// Create the game
		let mut game = Game{
			win: win,
			ren: ren,
			world: world,
			state: GameState::default(),
		};
		
		// Handle initial events
		if state.focused {
			game.handle_events(vec![InternalEvent::Focus]);
		} else {
			game.handle_events(vec![InternalEvent::Unfocus]);
		}
		
		Ok(game)
	}
	
	pub fn run(&mut self) -> GameResult<()> {
		// Show the window
		self.win.get_window().map(|w| w.show());
		
		while !self.state.quit {
			// Process events
			let es = self.process_events();
			
			// Update world & handle events
			self.handle_events(es);
			// Center the cursor if focused
			if self.state.focused {
				if let Some(win) = self.win.get_window() {
					win.get_outer_size()
						.map(|(w, h)| win.set_cursor_position(w as i32 / 2, h as i32 / 2));
				}
			}
			
			// Render world
			let mut frame = self.win.draw();
			// TODO: self.world.render(&mut self.ren);
			
			// Swap buffers
			frame.finish()
				.map_err(|e| writeln!(io::stderr(), "warning: swap_buffers failed: {}", e))
				.ok();
		}
		
		Ok(())
	}
	
	/// Process external events into internal events.
	pub fn process_events(&mut self) -> Vec<InternalEvent> {
		InternalEvent::from_events(&self.state, &mut self.win.poll_events())
	}
	
	/// Handle internal events
	pub fn handle_events(&mut self, es: Vec<InternalEvent>) {
		use event::InternalEvent::*;
		use glutin::CursorState;
		
		for e in es {
			println!("event recieved: {:?}", e);
			match e {
				Quit => {
					self.state.quit = true;
				},
				Move(v) => {
					self.world.move_player(v);
				},
				Focus => {
					self.state.focused = true;
					if let Some(win) = self.win.get_window() {
						win.set_cursor_state(CursorState::Grab)
							.map_err(|e| writeln!(io::stderr(), "warning: set_cursor_state failed: {}", e)).ok();
					}
				},
				Unfocus => {
					self.state.focused = false;
					if let Some(win) = self.win.get_window() {
						win.set_cursor_state(CursorState::Normal)
							.map_err(|e| writeln!(io::stderr(), "warning: set_cursor_state failed: {}", e)).ok();
					}
				}
			}
		}
	}
}
