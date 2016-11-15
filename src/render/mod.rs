#[allow(unused_imports)]
use prelude::*;

use std::rc::Rc;

use glium::{Depth, DepthTest, Frame, Program, Surface};
use glium::draw_parameters::{DrawParameters, BackfaceCullingMode};
use glium::index::{PrimitiveType, NoIndices};
use glium::backend::Context;
use glium::uniforms::MinifySamplerFilter;
use glium::uniforms::MagnifySamplerFilter;
use glium::uniforms::SamplerWrapFunction;

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
	
	trace!("id: {}", id);
	
	if id.contains(is_separator) {
		let mut ret = String::with_capacity(id.len() + 1);
		for sub in id.split(is_separator) {
			ret.push_str(sub);
			ret.push(MAIN_SEPARATOR);
		}
		ret.pop();
		trace!("ret: {}", ret);
		ret
	} else {
		trace!("ret: {}", id);
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
	mat_view: Mat4,
}
impl Render {
	pub fn new(ctx: Rc<Context>, c: Camera) -> GameResult<Render> {
		let mat_view = c.view_matrix();
		Ok(Render {
			ctx: ctx.clone(),
			mesh_bank: MeshBank::new(ctx.clone()),
			tex_bank: TextureBank::new(ctx.clone())?,
			phong_program: parse::load_shader_program(&ctx, "res/shader/phong")?,
			camera: c,
			mat_view: mat_view,
		})
	}
	
	pub fn set_camera(&mut self, c: Camera) {
		self.mat_view = c.view_matrix();
		self.camera = c;
	}
	
	pub fn draw_mesh(&mut self, f: &mut Frame, mesh_id: MeshID, mat_model: Mat4) {
		let dims = f.get_dimensions();
		let mat_projection = self.camera.projection_matrix(dims.0, dims.1);
		let mat_mvp = mat_projection * self.mat_view * mat_model;
		// TODO: Get a default mesh if failed to load mesh_id
		let mesh = self.mesh_bank.get_mesh(mesh_id.clone()).unwrap();
		let tex = self.tex_bank.get_texture_or_default(mesh.material.map_Kd.clone().unwrap_or_else(|| String::new()));
		let ret = f.draw(
			&mesh.vertices,
			NoIndices(PrimitiveType::TrianglesList),
			&self.phong_program,
			&uniform! {
				u_mvp: array4x4(mat_mvp),
				Ka: array3(mesh.material.Ka),
				d: mesh.material.d,
				map_Ka: tex.sampled()
					.minify_filter(MinifySamplerFilter::Nearest)
					.magnify_filter(MagnifySamplerFilter::Nearest)
					.wrap_function(SamplerWrapFunction::Clamp)
					.anisotropy(1),
			},
			&DrawParameters {
				depth: Depth {
					test: DepthTest::IfLessOrEqual,
					write: true,
					..Default::default()
				},
				multisampling: false,
				dithering: false,
				backface_culling: BackfaceCullingMode::CullClockwise,
				..Default::default()
			}
		);
		ret.map_err(|e| warn!("Could not draw mesh {}: {}", mesh_id, e)).ok();
	}
}

#[derive(Debug, Clone)]
pub struct Material {
	/// Ambient colour
	pub Ka: Vec3,
	/// Difuse colour (TODO)
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
	/// Diffuse texture map (TODO)
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