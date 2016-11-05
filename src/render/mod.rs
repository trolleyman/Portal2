use prelude::*;

use self::mesh::MeshBank;

use std::rc::Rc;

use glium::backend::Context;

pub mod mesh;
pub mod parse;
pub mod texture;

pub struct Render {
	pub ctx: Rc<Context>,
	pub mesh_bank: MeshBank,
}
impl Render {
	pub fn new(ctx: Rc<Context>) -> Result<Render, String> {
		Ok(Render {
			ctx: ctx.clone(),
			mesh_bank: MeshBank::new(ctx),
		})
	}
}
