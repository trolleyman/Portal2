#[allow(unused_imports)]
use prelude::*;

use world::entity::Portal;

use std::rc::Rc;

use glium::{Depth, DepthTest, Frame, Program, Surface, StencilTest, StencilOperation};
use glium::draw_parameters::{Stencil, DrawParameters, BackfaceCullingMode};
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

const SHADER_PHONG: &'static str = "res/shader/phong";
const SHADER_UNLIT: &'static str = "res/shader/unlit";

pub struct Render {
	#[allow(dead_code)]
	ctx: Rc<Context>,
	mesh_bank: MeshBank,
	tex_bank: TextureBank,
	phong_program: Program,
	unlit_program: Program,
	camera_org: Camera,
	light: Light,
	portals: Option<[Portal; 2]>,
	camera_trans: Camera,
}
impl Render {
	pub fn new(ctx: Rc<Context>, c: Camera, l: Light) -> GameResult<Render> {
		Ok(Render {
			ctx: ctx.clone(),
			mesh_bank: MeshBank::new(ctx.clone())?,
			tex_bank: TextureBank::new(ctx.clone())?,
			phong_program: parse::load_shader_program(&ctx, SHADER_PHONG)?,
			unlit_program: parse::load_shader_program(&ctx, SHADER_UNLIT)?,
			camera_org: c,
			light: l,
			portals: None,
			camera_trans: c,
		})
	}
	
	pub fn reload_meshes(&mut self) {
		self.mesh_bank.clear_cache();
		self.mesh_bank.load_meshes();
	}
	
	pub fn reload_textures(&mut self) {
		self.tex_bank.clear_cache();
		self.tex_bank.load_textures();
	}
	
	pub fn reload_shaders(&mut self) {
		match parse::load_shader_program(&self.ctx, SHADER_PHONG) {
			Ok(p) => self.phong_program = p,
			Err(e) => warn!("Could not reload shader '{}': {}", SHADER_PHONG, e),
		}
		match parse::load_shader_program(&self.ctx, SHADER_UNLIT) {
			Ok(p) => self.unlit_program = p,
			Err(e) => warn!("Could not reload shader '{}': {}", SHADER_UNLIT, e),
		}
	}
	
	pub fn set_light(&mut self, l: Light) {
		self.light = l;
	}
	
	pub fn set_camera(&mut self, c: Camera) {
		self.camera_org = c;
		self.camera_trans = c;
	}
	
	pub fn set_portals(&mut self, f: &mut Frame, portal_from: Portal, portal_to: Portal) {
		if let Some([portal_from, _]) = self.portals {
			// Clear the stencil
			f.clear_stencil(0);
			// Draw depth too
			self.camera_trans = self.camera_org.clone();
			self.draw_portal_stencil(f, portal_from, true, false);
		}
		self.portals = Some([portal_from, portal_to]);
		self.camera_trans = self.camera_org.clone();
		// Draw stencil
		self.draw_portal_stencil(f, portal_from, false, true);
		// Transform camera
		self.camera_trans.transform_by_portal(portal_from, portal_to);
	}
	
	pub fn unset_portals(&mut self, f: &mut Frame) {
		if let Some([portal_from, _]) = self.portals {
			// Clear the stencil
			f.clear_stencil(0);
			// Draw depth too
			self.camera_trans = self.camera_org;
			self.draw_portal_stencil(f, portal_from, true, false);
		}
		self.portals = None;
		self.camera_trans = self.camera_org;
	}
	
	pub fn draw_portal_stencil(&mut self, f: &mut Frame, p: Portal, write_depth: bool, write_stencil: bool) {
		let dims = f.get_dimensions();
		let mesh = self.mesh_bank.portal_stencil_mesh();
		let mat_model = p.model_matrix();
		let mat_projection = self.camera_trans.projection_matrix(dims.0, dims.1);
		let mat_mvp = mat_projection * self.camera_trans.view_matrix() * mat_model;
		let map = Render::get_tex(&mut self.tex_bank, None);
		f.draw(
			&mesh.vertices,
			mesh.indices_source(),
			&self.unlit_program,
			&uniform! {
				u_mvp: array4x4(mat_mvp),
				u_model_mat: array4x4(mat_model),
				u_color: array3(mesh.material.Ka),
				u_d: mesh.material.d,
				u_map: Render::sample_tex(&map.0),
				u_map_uv_scale: array2(map.1.uv_scale),
			},
			&DrawParameters {
				depth: Depth {
					test: if write_depth { DepthTest::Overwrite } else { DepthTest::IfLessOrEqual },
					write: write_depth,
					..Default::default()
				},
				stencil: Stencil {
					reference_value_counter_clockwise: 1,
					fail_operation_counter_clockwise: StencilOperation::Keep,
					pass_depth_fail_operation_counter_clockwise: StencilOperation::Keep,
					depth_pass_operation_counter_clockwise: if write_stencil { StencilOperation::Replace } else { StencilOperation::Keep },
					..Default::default()
				},
				color_mask: (false, false, false, false),
				backface_culling: BackfaceCullingMode::CullClockwise,
				..Default::default()
			}
		).map_err(|e| warn!("Could not draw portal stencil: {}", e)).ok();
	}
	
	fn get_tex(tex_bank: &mut TextureBank, id: Option<(TextureID, TextureOptions)>) -> (Rc<Texture2d>, TextureOptions) {
		if let Some((id, opt)) = id {
			(tex_bank.get_texture_or_error(id), opt)
		} else {
			(tex_bank.default_texture(), TextureOptions::default())
		}
	}
	
	fn sample_tex<'a>(t: &'a Rc<Texture2d>) -> Sampler<'a, Texture2d> {
		t.sampled()
			.minify_filter(MinifySamplerFilter::Nearest)
			.magnify_filter(MagnifySamplerFilter::Nearest)
	}
	
	pub fn draw_mesh(&mut self, f: &mut Frame, mesh_id: MeshID, mat_model: Mat4) {
		let dims = f.get_dimensions();
		let mat_projection = self.camera_trans.projection_matrix(dims.0, dims.1);
		let mat_mvp = mat_projection * self.camera_trans.view_matrix() * mat_model;
		// TODO: Get a default mesh if failed to load mesh_id
		let mesh = self.mesh_bank.get_mesh_or_default(mesh_id.clone());
		let map_Ka = Render::get_tex(&mut self.tex_bank, mesh.material.get_map_Ka());
		let map_Kd = Render::get_tex(&mut self.tex_bank, mesh.material.map_Kd.clone());
		
		self.draw_mesh_ext(f, mesh, mat_mvp, mat_model, map_Ka, map_Kd)
			.map_err(|e| warn!("Could not draw mesh '{}': {}", mesh_id, e)).ok();
	}
	
	fn draw_mesh_ext(&mut self, f: &mut Frame, mesh: Rc<Mesh>, mat_mvp: Mat4, mat_model: Mat4, map_Ka: (Rc<Texture2d>, TextureOptions), map_Kd: (Rc<Texture2d>, TextureOptions)) -> GameResult<()> {
		let stencil = if self.portals.is_some() {
			Stencil {
				reference_value_counter_clockwise: 1,
				test_counter_clockwise: StencilTest::IfEqual{ mask: 0xFFFFFFFF },
				..Default::default()
			}
		} else {
			Default::default()
		};
		let params = DrawParameters {
			depth: Depth {
				test: DepthTest::IfLessOrEqual,
				write: true,
				..Default::default()
			},
			stencil: stencil,
			backface_culling: BackfaceCullingMode::CullClockwise,
			..Default::default()
		};
		let ret = if mesh.material.lighting_disabled {
			f.draw(
				&mesh.vertices,
				mesh.indices_source(),
				&self.unlit_program,
				&uniform! {
					u_mvp: array4x4(mat_mvp),
					u_model_mat: array4x4(mat_model),
					u_color: array3(mesh.material.Ka),
					u_d: mesh.material.d,
					u_map: Render::sample_tex(&map_Ka.0),
					u_map_uv_scale: array2(map_Ka.1.uv_scale),
				},
				&params
			)
		} else {
			f.draw(
				&mesh.vertices,
				mesh.indices_source(),
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
					u_map_Ka: Render::sample_tex(&map_Ka.0),
					u_map_Ka_uv_scale: array2(map_Ka.1.uv_scale),
					u_map_Kd: Render::sample_tex(&map_Kd.0),
					u_map_Kd_uv_scale: array2(map_Kd.1.uv_scale),
				},
				&params
			)
		};
		ret.map_err(|e| format!("{}", e))
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
	/// Whether lighting is disabled for the objects
	pub lighting_disabled: bool,
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
	pub map_Ka: Option<(TextureID, TextureOptions)>,
	/// Diffuse texture map
	pub map_Kd: Option<(TextureID, TextureOptions)>,
	/// Specular color texture map (TODO)
	pub map_Ks: Option<(TextureID, TextureOptions)>,
	/// Emissive texture map (TODO)
	pub map_Ke: Option<(TextureID, TextureOptions)>,
	/// Specular highlight component texture map (TODO)
	pub map_Ns: Option<(TextureID, TextureOptions)>,
	/// Alpha texture map (TODO)
	pub map_d: Option<(TextureID, TextureOptions)>,
	/// Bump map (TODO)
	pub bump: Option<(TextureID, TextureOptions)>,
	/// Displacement map (TODO)
	pub disp: Option<(TextureID, TextureOptions)>,
}
impl Material {
	/// Getting the ambient texture map requires hacky logic.
	/// 
	/// For objects with a diffuse texture map, we want it to be that.
	/// For all other objects, we want it to be the default texture.
	fn get_map_Ka(&self) -> Option<(TextureID, TextureOptions)> {
		if self.map_Ka.is_some() {
			self.map_Ka.clone()
		} else if self.map_Kd.is_some() {
			self.map_Kd.clone()
		} else {
			None
		}
	}
}
impl Default for Material {
	fn default() -> Material {
		Material {
			lighting_disabled: false,
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