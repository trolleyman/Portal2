use prelude::*;

use std::fs::File;
use std::rc::Rc;
use std::collections::HashMap;

use glium::backend::Context;
use glium::texture::{ClientFormat, RawImage2d, Texture2d};

use vfs;
use png::{self, Parameter};

pub type TextureID = String;

pub struct TextureBank {
	ctx: Rc<Context>,
	cache: HashMap<TextureID, Texture2d>,
}

impl TextureBank {
	pub fn new(ctx: Rc<Context>) -> TextureBank {
		TextureBank {
			ctx: ctx,
			cache: HashMap::new(),
		}
	}
	
	/// Gets a teture from the TextureBank
	pub fn get_texture<'a>(&'a mut self, id: TextureID) -> GameResult<&'a Texture2d> {
		// If cache doesn't exist, loads it from a file.
		if self.cache.get(&id).is_none() {
			self.cache.insert(id.clone(), tex_from_file(&self.ctx, &id)?);
		}
		Ok(self.cache.get(&id).unwrap())
	}
	
	/// Loads a texture into the TextureBank
	pub fn load_texture(&mut self, id: TextureID) -> GameResult<()> {
		self.get_texture(id).map(|_| ())
	}
}

fn tex_from_file(ctx: &Rc<Context>, id: &TextureID) -> GameResult<Texture2d> {
	let path = vfs::canonicalize_exe(id);
	let f = File::open(&path)
		.map_err(|e| format!("Invalid png file ({}): {}", e, path.display()))?;
	
	let mut decoder = png::Decoder::new(f);
	(png::TRANSFORM_EXPAND | png::TRANSFORM_STRIP_ALPHA).set_param(&mut decoder);
	let (info, mut reader) = decoder.read_info()
		.map_err(|e| format!("Invalid png file ({}): {}", e, path.display()))?;
	
	let mut buf = Vec::with_capacity(info.buffer_size());
	reader.next_frame(&mut buf)
		.map_err(|e| format!("Invalid png file ({}): {}", e, path.display()))?;
	
	let raw = RawImage2d {
		data: buf.into(),
		width: info.width,
		height: info.height,
		format: ClientFormat::U8U8U8,
	};
	
	let tex = Texture2d::new(ctx, raw)
		.map_err(|e| format!("Invalid png file ({}): {}", e, path.display()))?;
	Ok(tex)
}
