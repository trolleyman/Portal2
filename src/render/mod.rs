#[allow(unused_imports)]
use prelude::*;

use std::rc::Rc;

use glium::{Depth, DepthTest, Frame, Program, Surface};
use glium::draw_parameters::{DrawParameters, BackfaceCullingMode};
use glium::index::{PrimitiveType, NoIndices};
use glium::backend::Context;
use glium::uniforms::MinifySamplerFilter;
use glium::uniforms::MagnifySamplerFilter;
use glium::uniforms::Sampler;
use glium::texture::Texture2d;

pub use self::camera::Camera;
pub use self::mesh::*;
pub use self::texture::*;

mod camera;
mod mesh;
mod parse;
mod texture;

fn normalize_id(id: String) -> String {
	use std::path::MAIN_SEPARATOR;
	use std::path::is_separator;
		
	if id.contains(is_separator) {
		let mut ret = String::with_capacity(id.len() + 1);
		for sub in id.split(is_separator) {
			ret.push_str(sub);
			ret.push(MAIN_SEPARATOR);
		}
		ret.pop();
		ret
	} else {
		id
	}
}

pub struct Render {
	#[allow(dead_code)]
	ctx: Rc<Context>,
	mesh_bank: MeshBank,
	tex_bank: TextureBank,
	phong_program: Program,
	camera: Camera,
	light: Light,
	mat_view: Mat4,
}
impl Render {
	pub fn new(ctx: Rc<Context>, c: Camera, l: Light) -> GameResult<Render> {
		let mat_view = c.view_matrix();
		Ok(Render {
			ctx: ctx.clone(),
			mesh_bank: MeshBank::new(ctx.clone())?,
			tex_bank: TextureBank::new(ctx.clone())?,
			phong_program: parse::load_shader_program(&ctx, "res/shader/phong")?,
			camera: c,
			light: l,
			mat_view: mat_view,
		})
	}
	
	pub fn set_light(&mut self, l: Light) {
		self.light = l;
	}
	
	pub fn set_camera(&mut self, c: Camera) {
		self.mat_view = c.view_matrix();
		self.camera = c;
	}
	
	pub fn draw_mesh(&mut self, f: &mut Frame, mesh_id: MeshID, mat_model: Mat4) {
		fn get_tex(tex_bank: &mut TextureBank, id: Option<TextureID>) -> Rc<Texture2d> {
			if let Some(id) = id {
				tex_bank.get_texture_or_error(id)
			} else {
				tex_bank.default_texture()
			}
		}
		fn sample_tex<'a>(t: &'a Rc<Texture2d>) -> Sampler<'a, Texture2d> {
			t.sampled()
				.minify_filter(MinifySamplerFilter::Nearest)
				.magnify_filter(MagnifySamplerFilter::Nearest)
		}
		
		let dims = f.get_dimensions();
		let mat_projection = self.camera.projection_matrix(dims.0, dims.1);
		let mat_mvp = mat_projection * self.mat_view * mat_model;
		// TODO: Get a default mesh if failed to load mesh_id
		let mesh = self.mesh_bank.get_mesh_or_default(mesh_id.clone());
		let map_Ka = get_tex(&mut self.tex_bank, mesh.material.map_Ka.clone());
		let map_Kd = get_tex(&mut self.tex_bank, mesh.material.map_Kd.clone());
		let ret = f.draw(
			&mesh.vertices,
			NoIndices(PrimitiveType::TrianglesList),
			&self.phong_program,
			&uniform! {
				u_light_ambient: array4(self.light.ambient),
				u_light_diffuse: array4(self.light.diffuse),
				u_light_pos: array3(self.light.pos),
				u_mvp: array4x4(mat_mvp),
				u_model_mat: array4x4(mat_model),
				u_Ka: array3(mesh.material.Ka),
				u_Kd: array3(mesh.material.Kd),
				u_d: mesh.material.d,
				u_map_Ka: sample_tex(&map_Ka),
				u_map_Kd: sample_tex(&map_Kd),
			},
			&DrawParameters {
				depth: Depth {
					test: DepthTest::IfLessOrEqual,
					write: true,
					..Default::default()
				},
				backface_culling: BackfaceCullingMode::CullClockwise,
				..Default::default()
			}
		);
		ret.map_err(|e| warn!("Could not draw mesh {}: {}", mesh_id, e)).ok();
	}
}

#[derive(Debug, Copy, Clone)]
pub struct Light {
	/// Ambient RGBA intensity
	pub ambient: Vec4,
	/// Diffuse RGBA intensity
	pub diffuse: Vec4,
	/// Specular RGBA intensity (TODO)
	pub specular: Vec4,
	/// (x, y, z, w) position of light. (TODO: for now, just a position of a spot light)
	/// 
	/// If w == 0, then the light is directional, otherwise it is positional.
	pub pos: Vec3,
	/// (x, y, z) direction of light (TODO)
	pub spot_direction: Vec3,
	/// Spotlight exponent (TODO)
	pub spot_exponent: Flt,
	/// Spotlight cutoff angle (TODO)
	pub spot_cutoff: Flt,
	/// Constant attenuation factor (TODO)
	pub constant_attenuation: Flt,
	/// Linear attenuation factor (TODO)
	pub linear_attenuation: Flt,
	/// Quadratic attenuation factor (TODO)
	pub quadratic_attenuation: Flt,
}
impl Default for Light {
	fn default() -> Light {
		Light {
			ambient: vec4(0.0, 0.0, 0.0, 1.0),
			diffuse: vec4(1.0, 1.0, 1.0, 1.0),
			specular: vec4(1.0, 1.0, 1.0, 1.0),
			pos: vec3(0.0, 0.0, 1.0/*, 0.0*/),
			spot_direction: vec3(0.0, 0.0, -1.0),
			spot_exponent: 0.0,
			spot_cutoff: 180.0,
			constant_attenuation: 1.0,
			linear_attenuation: 0.0,
			quadratic_attenuation: 0.0,
		}
	}
}


#[derive(Debug, Clone)]
pub struct Material {
	/// Ambient colour TODO: Convert to vec4 (to include alpha)
	pub Ka: Vec3,
	/// Difuse colour
	pub Kd: Vec3,
	/// Specular colour (TODO)
	pub Ks: Vec3,
	/// Emissive colour (TODO)
	pub Ke: Vec3,
	/// Specular exponent (TODO)
	pub Ns: Flt,
	/// Transparency
	pub d: Flt,
	/// Ambient texture map
	pub map_Ka: Option<TextureID>,
	/// Diffuse texture map
	pub map_Kd: Option<TextureID>,
	/// Specular color texture map (TODO)
	pub map_Ks: Option<TextureID>,
	/// Emissive texture map (TODO)
	pub map_Ke: Option<TextureID>,
	/// Specular highlight component texture map (TODO)
	pub map_Ns: Option<TextureID>,
	/// Alpha texture map (TODO)
	pub map_d: Option<TextureID>,
	/// Bump map (TODO)
	pub bump: Option<TextureID>,
	/// Displacement map (TODO)
	pub disp: Option<TextureID>,
}
impl Default for Material {
	fn default() -> Material {
		Material {
			Ka: vec3(0.7, 0.7, 0.7),
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

#[cfg(test)]
mod test {
	#[test]
	fn test_normalize_id() {
		macro_rules! tni {
			($input:expr, $exp_win:expr, $exp_oth:expr) => ({
				let input = String::from($input);
				let exp_win = $exp_win;
				let exp_oth = $exp_oth;
				let ret = super::normalize_id(input);
				if cfg!(windows) {
					assert_eq!(ret, exp_win);
				} else {
					assert_eq!(ret, exp_oth);
				}
			})
		}
		
		tni!("res/thing\\other/thing2", "res\\thing\\other\\thing2", "res/thing/other/thing2");
	}
}