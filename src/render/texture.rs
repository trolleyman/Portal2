use prelude::*;

use std::borrow::Cow;
use std::fs::File;
use std::rc::Rc;
use std::collections::HashMap;

use glium::backend::Context;
use glium::texture::{ClientFormat, RawImage2d, Texture2d};

use game::duration_to_millis;
use vfs;
use png::{self, Parameter};
use super::normalize_id;

pub type TextureID = String;

pub const TEX_DIR: &'static str = "res/tex/";

pub struct TextureBank {
	ctx: Rc<Context>,
	cache: HashMap<TextureID, GameResult<Rc<Texture2d>>>,
	default_texture: Rc<Texture2d>,
}

impl TextureBank {
	pub fn new(ctx: Rc<Context>) -> GameResult<TextureBank> {
		let data: &[f32] = &[1.0, 1.0, 1.0, 1.0];
		let dt = Texture2d::new(&ctx, RawImage2d {
			data: Cow::from(data),
			width: 1,
			height: 1,
			format: ClientFormat::F32F32F32F32,
		}).map_err(|e| format!("Unable to create TexureBank: Unable to create default texture: {}", e))?;
		
		let mut tb = TextureBank {
			ctx: ctx,
			cache: HashMap::new(),
			default_texture: Rc::new(dt),
		};
		tb.load_textures();
		Ok(tb)
	}
	
	/// Returns the default texture (one opaque white pixel)
	pub fn default_texture(&self) -> Rc<Texture2d> {
		self.default_texture.clone()
	}
	
	/// Load the texture from a file, or an error texture if that doesn't work.
	/// 
	/// The error texture is a pink and black checkerboard (TODO: For now it is the same as default_texture).
	pub fn get_texture_or_error(&mut self, id: TextureID) -> Rc<Texture2d> {
		self.get_texture_or_default(id)
	}
	
	/// Load the texture from a file, or the default if that doesn't work
	/// 
	/// The default texture is a white pixel. 
	pub fn get_texture_or_default(&mut self, id: TextureID) -> Rc<Texture2d> {
		self.get_texture(id.clone())
			.unwrap_or(self.default_texture())
	}
	
	/// Gets a teture from the TextureBank
	pub fn get_texture(&mut self, id: TextureID) -> GameResult<Rc<Texture2d>> {
		// Normalize id first
		let id = normalize_id(id);
		// If cache doesn't exist, loads it from a file.
		if self.cache.get(&id).is_none() {
			use std::time::Instant;
			let t_start = Instant::now();
			let res = match tex_from_file(&self.ctx, &id).map(|t| Rc::new(t)) {
				Ok(t) => {
					info!("Loaded texture: {} ({}ms)", &id, duration_to_millis(t_start.elapsed()));
					Ok(t)
				},
				Err(e) => {
					warn!("Could not load texture ({}): {}", &id, &e);
					Err(e)
				}
			};
			self.cache.insert(id.clone(), res);
		}
		self.cache.get(&id).unwrap().clone()
	}
	
	/// Loads a texture into the TextureBank
	pub fn load_texture(&mut self, id: TextureID) -> GameResult<()> {
		self.get_texture(id).map(|_| ())
	}
	
	/// Loads all of the textures in the TEX_DIR directory
	fn load_textures(&mut self) {
		use std::fs;
		use vfs;
		
		// Iterate over files in TEX_DIR
		let dir = vfs::canonicalize_exe(TEX_DIR);
		let it = match fs::read_dir(&dir) {
			Ok(it) => it,
			Err(e) => {
				warn!("Could not iterate over textures directory ({}): {}", dir.display(), e);
				return;
			}
		};
		
		// TODO: Clean up this code - ugly but works for now
		for file in it {
			match file {
				Ok(f) => {
					let id = TEX_DIR.to_string() + &f.file_name().to_string_lossy().into_owned();
					if !id.ends_with(".png") {
						continue;
					}
					self.load_texture(id.clone()).ok();
				},
				_ => {} // Ignore files that return an error when iterating over them
			}
		}
	}

}

fn tex_from_file(ctx: &Rc<Context>, id: &TextureID) -> GameResult<Texture2d> {
	let path = vfs::canonicalize_exe(id);
	let f = File::open(&path)
		.map_err(|e| format!("Invalid png file ({}): {}", e, path.display()))?;
	
	let mut decoder = png::Decoder::new(f);
	// Alpha stripped due to png crate limitations
	(png::TRANSFORM_EXPAND | png::TRANSFORM_STRIP_ALPHA).set_param(&mut decoder);
	let (info, mut reader) = decoder.read_info()
		.map_err(|e| format!("Invalid png file ({}): {}", e, path.display()))?;
	
	let mut buf = vec![0; info.buffer_size()];
	reader.next_frame(&mut buf)
		.map_err(|e| format!("Invalid png file ({}): {}", e, path.display()))?;
	
	let raw = RawImage2d {
		data: buf.into(),
		width: info.width,
		height: info.height,
		format: ClientFormat::U8U8U8, // 3 U8s because Alpha is stripped due to png crate limitations
	};
	
	let tex = Texture2d::new(ctx, raw)
		.map_err(|e| format!("Invalid png file ({}): {}", e, path.display()))?;
	Ok(tex)
}
