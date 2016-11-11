#[allow(unused_imports)]
use prelude::*;

use self::mesh::MeshBank;

use std::rc::Rc;

use glium::backend::Context;

use self::texture::{TextureID, TextureBank};

pub mod mesh;
pub mod parse;
pub mod texture;

pub struct Render {
	pub ctx: Rc<Context>,
	pub mesh_bank: MeshBank,
	pub tex_bank: TextureBank,
}
impl Render {
	pub fn new(ctx: Rc<Context>) -> Result<Render, String> {
		Ok(Render {
			ctx: ctx.clone(),
			mesh_bank: MeshBank::new(ctx.clone()),
			tex_bank: TextureBank::new(ctx),
		})
	}
}

#[derive(Debug, Clone)]
pub struct Material {
	/// Ambient colour
	pub Ka: Vec3,
	/// Difuse colour
	pub Kd: Vec3,
	/// Specular colour
	pub Ks: Vec3,
	/// Emissive colour
	pub Ke: Vec3,
	/// Specular exponent
	pub Ns: Flt,
	/// Transparency
	pub d: Flt,
	/// Ambient texture map
	pub map_Ka: Option<TextureID>,
	/// Diffuse texture map
	pub map_Kd: Option<TextureID>,
	/// Specular color texture map
	pub map_Ks: Option<TextureID>,
	/// Emissive texture map
	pub map_Ke: Option<TextureID>,
	/// Specular highlight component
	pub map_Ns: Option<TextureID>,
	/// Alpha texture map
	pub map_d: Option<TextureID>,
	/// Bump map
	pub bump: Option<TextureID>,
	/// Displacement map
	pub disp: Option<TextureID>,
}
impl Default for Material {
	fn default() -> Material {
		Material {
			Ka: vec3(1.0, 1.0, 1.0),
			Kd: vec3(1.0, 1.0, 1.0),
			Ks: vec3(0.0, 0.0, 0.0),
			Ke: vec3(0.0, 0.0, 0.0),
			Ns: 10.0,
			d: 1.0,
			map_Ka: None,
			map_Kd: None,
			map_Ks: None,
			map_Ke: None,
			map_Ns: None,
			map_d: None,
			bump: None,
			disp: None,
		}
	}
}
