use prelude::*;

use std::time::{Instant, Duration};

use glutin::WindowBuilder;
use glium::backend::Facade;
use glium::{Surface, Display, DisplayBuild};

use key::KeyboardState;
use event::InternalEvent;
use render::Render;
use world::World;

pub const WIN_INIT_W: u32 = 800;
pub const WIN_INIT_H: u32 = 600;

pub fn duration_to_secs(d: Duration) -> f64 {
	let secs = d.as_secs();
	let nanos = d.subsec_nanos();
	secs as f64 + nanos as f64 / 1000_000_000.0
}

pub fn duration_to_millis(d: Duration) -> u64 {
	let secs = d.as_secs();
	let nanos = d.subsec_nanos();
	secs * 1000 + nanos as u64 / 1000_000
}

pub struct GameState {
	/// Should the game quit?
	quit: bool,
	/// Is the game focused on?
	/// If yes, then the cursor should be reset to the middle of the window each loop.
	pub focused: bool,
	/// State of the keyboard
	pub keyboard_state: KeyboardState,
	/// Middle of the screen co-ordinates
	pub mid_screen: Option<(i32, i32)>,
	/// Number of frames to ignore the mouse for. If zero, don't ignore the mouse.
	pub ignore_mouse_frames: u32,
}
impl Default for GameState {
	fn default() -> GameState {
		GameState {
			quit: false,
			focused: false,
			keyboard_state: KeyboardState::new(),
			mid_screen: None,
			ignore_mouse_frames: 0,
		}
	}
}

#[allow(dead_code)]
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
			.with_dimensions(WIN_INIT_W, WIN_INIT_H)
			.with_vsync()
			.with_visibility(false)
			.build_glium()
			.map_err(|e| format!("Window creation error: {}", e))?;
		
		// Create the world
		let world = World::new()?;
		// And the renderer
		let ren = Render::new(win.get_context().clone(), world.camera().clone())?;
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
			game.handle_events(0.0, vec![InternalEvent::Focus]);
		} else {
			game.handle_events(0.0, vec![InternalEvent::Unfocus]);
		}
		
		Ok(game)
	}
	
	pub fn run(&mut self) -> GameResult<()> {
		// Show the window
		self.win.get_window().map(|w| w.show());
		
		let mut last_time = Instant::now();
		
		while !self.state.quit {
			// Calculate times
			let dt_dur = last_time.elapsed();
			let dt_millis = duration_to_millis(dt_dur);
			let dt = duration_to_secs(dt_dur) as Flt;
			last_time = Instant::now();
			
			info!("{:3}ms dt: {:.6} pos: {:?}", dt_millis, dt, self.world.camera().pos);
			
			// Process events
			let es = self.process_events();
			
			// Update world & handle events
			self.handle_events(dt, es);
			// Center the cursor if focused
			if self.state.focused {
				if let Some(win) = self.win.get_window() {
					if let Some((w, h)) = win.get_outer_size() {
						let mid = (w as i32 / 2, h as i32 / 2);
						trace!("Mid: {}, {}", mid.0, mid.1);
						self.state.mid_screen = Some(mid);
						win.set_cursor_position(mid.0, mid.1);
					}
				}
			}
			
			// Clear frame
			let mut frame = self.win.draw();
			frame.clear_all((0.0, 0.0, 0.0, 1.0), 1.0, 0);
			
			// Render world
			self.world.render(&mut self.ren, &mut frame);
			
			// Swap buffers
			frame.finish()
				.map_err(|e| warn!("swap_buffers failed: {}", e))
				.ok();
		}
		
		Ok(())
	}
	
	/// Process external events into internal events. Also update the KeyboardState
	pub fn process_events(&mut self) -> Vec<InternalEvent> {
		InternalEvent::from_events(&mut self.state, &mut self.win.poll_events())
	}
	
	/// Handle internal events
	pub fn handle_events(&mut self, dt: Flt, es: Vec<InternalEvent>) {
		use event::InternalEvent::*;
		use glutin::CursorState;
		
		const SPEED_MULT: Flt = 2.0;
		
		for e in es {
			info!("Event recieved: {:?}", e);
			match e {
				Quit => {
					self.state.quit = true;
				},
				Move(v) => {
					self.world.move_player(v * dt * SPEED_MULT);
				},
				Rotate(r) => {
					self.world.rotate_player(r);
				}
				Focus => {
					self.state.focused = true;
					self.state.ignore_mouse_frames = 1;
					if let Some(win) = self.win.get_window() {
						win.set_cursor_state(CursorState::Grab)
							.map_err(|e| warn!("set_cursor_state failed: {}", e)).ok();
					}
				},
				Unfocus => {
					self.state.focused = false;
					self.state.mid_screen = None;
					if let Some(win) = self.win.get_window() {
						win.set_cursor_state(CursorState::Normal)
							.map_err(|e| warn!("set_cursor_state failed: {}", e)).ok();
					}
				}
			}
		}
	}
}
