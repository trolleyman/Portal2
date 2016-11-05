use prelude::*;

use glutin::{Window, WindowBuilder};

use event::InternalEvent;
use render::Render;
use world::World;

pub struct GameState {
	/// Should the game quit?
	quit: bool,
	/// Is the game focused on?
	/// If yes, then the cursor should be reset to the middle of the window each loop.
	focused: bool,
}
impl Default for GameState {
	fn default() -> GameState {
		GameState {
			quit: false,
			focused: true,
		}
	}
}

pub struct Game {
	win: Window,
	ren: Render,
	world: World,
	state: GameState,
}
impl Game {
	pub fn new() -> GameResult<Game> {
		// Initialize the window
		let win = (WindowBuilder::new()
			.with_title("Portal")
			.with_dimensions(800, 600)
			.with_vsync()
			.with_visibility(false)
			.build_strict()
			.into() : GameResult<_>)?;
		
		unsafe { GameResult::from(win.make_current())?; }
		
		// And the renderer
		let ren = Render::new()?;
		// Create the world
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
			let _ = game.handle_events(vec![InternalEvent::Focus]);
		} else {
			let _ = game.handle_events(vec![InternalEvent::Unfocus]);
		}
		
		GameResult::ok(game)
	}
	
	pub fn run(&mut self) -> GameResult<()> {
		// Show the window
		self.win.show();
		
		while !self.state.quit {
			// Process events
			let es = self.process_events();
			
			// Update world & handle events
			self.handle_events(es);
			// Center the cursor if focused
			if self.state.focused {
				self.win.get_outer_size()
					.map(|(w, h)| self.win.set_cursor_position(w as i32 /2, h as i32/2));
			}
			
			// Render world
			// TODO: self.world.render(&mut self.ren);
		}
		
		GameResult::ok(()).into()
	}
	
	/// Process external events into internal events.
	pub fn process_events(&mut self) -> Vec<InternalEvent> {
		InternalEvent::from_events(&mut self.win.poll_events())
	}
	
	/// Handle internal events
	pub fn handle_events(&mut self, es: Vec<InternalEvent>) {
		use event::InternalEvent::*;
		use glutin::CursorState;
		
		for e in es {
			match e {
				Quit => {
					self.state.quit = true;
				},
				Move(v) => {
					self.world.move_player(v);
				},
				Focus => {
					self.win.set_cursor_state(CursorState::Grab);
				},
				Unfocus => {
					self.win.set_cursor_state(CursorState::Normal);
				}
			}
		}
	}
}
