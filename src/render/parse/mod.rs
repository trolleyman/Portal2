use prelude::*;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::iter::Peekable;

use render::{TextureID, TextureOptions, Material};

use glium::Program;
use glium::backend::Context;

mod util;

use vfs;

/// Face is a series of 3 points representing a triangle
/// 
/// f 1/2/3 4/5/6 7/8/9 == vec3(vec3(1,2,3), vec3(4,5,6), vec3(7,8,9))
#[derive(Copy, Clone, Debug)]
pub struct IndexInfo {
	pub vert: Idx,
	pub uv: Idx,
	pub norm: Idx,
}
impl IndexInfo {
	pub fn new(vert: Idx, uv: Idx, norm: Idx) -> IndexInfo {
		IndexInfo {
			vert: vert,
			uv: uv,
			norm: norm,
		}
	}
}
#[derive(Copy, Clone, Debug)]
pub struct PreIndexInfo {
	pub vert: Idx,
	pub uv: Option<Idx>,
	pub norm: Option<Idx>,
}
impl PreIndexInfo {
	pub fn new(vert: Idx, uv: Option<Idx>, norm: Option<Idx>) -> PreIndexInfo {
		PreIndexInfo {
			vert: vert,
			uv: uv,
			norm: norm,
		}
	}
}

#[derive(Debug)]
pub struct ObjFile {
	/// Filepath (relative to current exe pos) e.g. "res/mesh/test.obj"
	pub rel_path: String,
	/// Absolute filepath
	pub path: PathBuf,
	/// Name of the object
	pub name: Option<String>,
	/// Materials referenced
	pub materials: HashMap<String, Material>,
	/// Material used for the object
	pub material: Option<String>,
	/// Vertices read
	pub vertices: Vec<Vec3>,
	/// Uvs read
	pub uvs: Vec<Vec2>,
	/// Vertex normals read
	pub normals: Vec<Vec3>,
	/// Faces read
	/// 
	/// Guaranteed to be valid (every index points to a vertex/uv/normal that has been read)
	pub faces: Vec<Vector3<IndexInfo>>,
	pub pre_faces: Vec<Vector3<PreIndexInfo>>,
}
impl ObjFile {
	pub fn new(rel_path: String) -> GameResult<ObjFile> {
		let mut f = ObjFile {
			rel_path: rel_path.clone(),
			path: vfs::canonicalize_exe(&rel_path),
			name: None,
			materials: HashMap::new(),
			material: None,
			vertices: vec![],
			uvs: vec![],
			normals: vec![],
			faces: vec![],
			pre_faces: vec![],
		};
		
		parse_file(&mut f)?;
		
		// Convert co-ordinate system from Blender co-ordinates (Z up, -Y forward) to our co-ordinates (Y up, -Z forward)
		f.convert_co_ordinates();
		
		// Validate pre_faces, so that we know all indices are in bounds
		f.validate()?;
		
		// Calculate faces from pre_faces
		f.calculate_faces();
		
		Ok(f)
	}
	
	/// Convert co-ordinate system from Blender co-ordinates (Z up, -Y forward) to our co-ordinates (Y up, -Z forward)
	fn convert_co_ordinates(&mut self) {
		// Currently don't do anything, see what happens
	}
	
	/// Calculates the faces from pre_faces.
	/// 
	/// To calculate missing uvs, we are using a flat generating algorithm.
	fn calculate_faces(&mut self) {
		trace!("Calculating faces...");
		if self.pre_faces.is_empty() {
			return; // Return if there are no faces to calculate
		}
		// Reserve space for faces
		let add = self.pre_faces.len() as isize - self.faces.len() as isize;
		self.faces.reserve( if add < 0 { 0 } else { add as usize } );
		
		// Get extents of mesh
		let mut min = self.vertices[self.pre_faces[0].x.vert as usize];
		let mut max = self.vertices[self.pre_faces[0].x.vert as usize];
		for f in self.pre_faces.iter() {
			for vi in [f.x, f.y, f.z].iter() {
				let v = self.vertices[vi.vert as usize];
				min = vec3(min.x.min(v.x), min.y.min(v.y), min.z.min(v.z));
				max = vec3(max.x.max(v.x), max.y.max(v.y), max.z.max(v.z));
			}
		}
		
		// Process faces
		for f in self.pre_faces.iter() {
			let v0 = self.vertices[f.x.vert as usize];
			let v1 = self.vertices[f.y.vert as usize];
			let v2 = self.vertices[f.z.vert as usize];
			
			// Calculate normals
			let normals = if f.x.norm.is_none() || f.y.norm.is_none() || f.z.norm.is_none() {
				let n = (v1 - v0).cross(v2 - v0).normalize();
				self.normals.push(n);
				let i = (self.normals.len() - 1) as Idx;
				vec3(f.x.norm.unwrap_or(i), f.y.norm.unwrap_or(i), f.z.norm.unwrap_or(i))
			} else {
				vec3(f.x.norm.unwrap()    , f.y.norm.unwrap()    , f.z.norm.unwrap()    )
			};
			// Calculate uvs
			let calc_prop = |f, min, max| { (f - min) / (min - max) };
			let calc_uv = |v: Vec3| { vec2(calc_prop(v.x, min.x, max.x), calc_prop(v.z, min.z, max.z)) };
			let calc_uv_idx = |uvs: &mut Vec<_>, v| { uvs.push(calc_uv(v)); uvs.len() as Idx - 1 };
			
			let uvs = &mut self.uvs;
			let uvs = if f.x.uv.is_none() || f.y.uv.is_none() || f.z.uv.is_none() {
				let uv0 = f.x.uv.unwrap_or_else(|| calc_uv_idx(uvs, v0));
				let uv1 = f.y.uv.unwrap_or_else(|| calc_uv_idx(uvs, v1));
				let uv2 = f.z.uv.unwrap_or_else(|| calc_uv_idx(uvs, v2));
				vec3(uv0, uv1, uv2)
			} else {
				vec3(f.x.uv.unwrap(), f.y.uv.unwrap(), f.z.uv.unwrap())
			};
			
			let face = vec3(
				IndexInfo::new(f.x.vert, uvs.x, normals.x),
				IndexInfo::new(f.y.vert, uvs.y, normals.y),
				IndexInfo::new(f.z.vert, uvs.z, normals.z));
			self.faces.push(face);
		}
	}
	
	/// Validate the file
	fn validate(&self) -> GameResult<()> {
		// Check if material is one of the materials read
		if let Some(ref m) = self.material {
			if self.materials.get(m).is_none() {
				return Err(format!("Invalid obj file ({}): Unknown material `{}`", self.rel_path, m));
			}
		}
		
		// Check if all pre_faces are valid
		for (i, f) in self.pre_faces.iter().enumerate() {
			fn check_index_info(i: usize, o: &ObjFile, ii: PreIndexInfo) -> GameResult<()> {
				if ii.vert as usize >= o.vertices.len() {
					return Err(format!("Invalid obj file ({}): Invalid face (index {}) vertex index: {}", &o.rel_path, i, ii.vert));
				} else if ii.uv.is_some() && ii.uv.unwrap() as usize >= o.uvs.len() {
					return Err(format!("Invalid obj file ({}): Invalid face (index {}) uv index: {}", &o.rel_path, i, ii.uv.unwrap()));
				} else if ii.norm.is_some() && ii.norm.unwrap() as usize >= o.normals.len() {
					return Err(format!("Invalid obj file ({}): Invalid face (index {}) normal index: {}", &o.rel_path, i, ii.norm.unwrap()));
				}
				Ok(())
			}
			check_index_info(i, self, f.x)?;
			check_index_info(i, self, f.y)?;
			check_index_info(i, self, f.z)?;
		}
		if self.material.is_none() {
			warn!("Object file loaded without a material: {}", &self.rel_path);
		}
		Ok(())
	}
}

#[derive(Debug)]
pub struct ParseState {
	command: String,
	lno: usize,
	path: PathBuf,
	rel_path: PathBuf,
}
impl ParseState {
	pub fn new(command: String, lno: usize, path: PathBuf, rel_path: PathBuf) -> ParseState {
		ParseState {
			command: command,
			lno: lno,
			path: path,
			rel_path: rel_path,
		}
	}
	pub fn to_error(&self) -> String {
		format!("Invalid command format `{}` at {}:{}", self.command, self.path.display(), self.lno)
	}
}

fn parse_file(f: &mut ObjFile) -> GameResult<()> {
	let mut s = String::new();
	File::open(&f.path)
		.map_err(|e| format!("Invalid obj file ({}): {}", e, &f.rel_path))?
		.read_to_string(&mut s)
		.map_err(|e| format!("Invalid obj file ({}): {}", e, &f.rel_path))?;
	
	parse_string(f, s)
}

fn parse_string(f: &mut ObjFile, s: String) -> GameResult<()> {
	// Get an iterator that ignores comments and empty lines
	let li = s.lines()
		.map(|l| l.split("#").next().unwrap_or(""));
	
	let mut state = ParseState::new(String::new(), 0, f.path.clone(), PathBuf::from(&f.rel_path));
	
	for (lno, line) in li.enumerate().map(|(lno, l)| (lno + 1, l)) {
		if line == "" { continue; }
		let mut args = line.split_whitespace().peekable();
		let command = args.next().unwrap_or("");
		state.command = command.to_string();
		state.lno = lno;
		trace!("Obj State: {:?}", state);
		match command {
			"mtllib" => {
				let mtl_rel_path = args.next()
					.ok_or_else(|| state.to_error())?;
				
				let mtl_rel_exe_path = util::remove_parents(&Path::new(&f.rel_path).join("..").join(&mtl_rel_path));
				trace!("mtl_rel_exe_path: {}", mtl_rel_exe_path.display());
				
				// Get the path of the mtl lib, as it is relative to the current file.
				let mut mtl_path = f.path.clone();
				mtl_path.pop();
				let mtl_path = mtl_path.join(mtl_rel_path);
				// Load the mtl file
				let mut mtl_s = String::new();
				File::open(&mtl_path)
					.and_then(|mut f| f.read_to_string(&mut mtl_s))
					.map_err(|e| format!("Invalid mtl file ({}): {}", e, mtl_path.display()))?;
				parse_mtl_string(f, &mtl_path, &mtl_rel_exe_path, &mtl_s)?;
			},
			"o" => {
				if f.name.is_some() {
					warn!("Object name redefined at location {}:{}", lno, f.rel_path);
				}
				f.name = Some(util::parse1_only(&state, &mut args)?);
			},
			"v" => {
				let v = util::parse_vec3_only(&state, &mut args)?;
				f.vertices.push(v);
			},
			"vt" => {
				let v = util::parseN_only(&state, 2, &mut args)?;
				f.uvs.push(vec2(v[0], v[1]));
			},
			"vn" => {
				let v = util::parse_vec3_only(&state, &mut args)?;
				f.normals.push(v.normalize());
			},
			"f" => {
				fn process_index(i: isize, l: usize) -> u32 {
					// Negative indices refer to the end of the array
					// We need to subtract 1 if it is positive, because we count from 0 wheras .obj is from 1.
					(if i >= 0 { i as usize - 1 } else { l - ((-i) as usize) }) as u32
				}
				fn process_index_info(state: &ParseState, f: &ObjFile, s: &str) -> GameResult<PreIndexInfo> {
					let mut iit = s.split("/");
					let str_vert = iit.next().ok_or_else(|| state.to_error())?;
					let str_uv = iit.next();
					let str_norm = iit.next();
					// Ensure that "1/2/3/" fails
					if iit.next().is_some() { return Err(state.to_error()); }
					
					let idx_vert = str_vert.parse().map_err(|_| state.to_error())?;
					let idx_vert = process_index(idx_vert, f.vertices.len());
					let idx_uv = match str_uv {
						None | Some("") => None,
						Some(s) => {
							let idx = s.parse().map_err(|_| state.to_error())?;
							Some(process_index(idx, f.uvs.len()))
						}
					};
					let idx_norm = match str_norm {
						None | Some("") => None,
						Some(s) => {
							let idx = s.parse().map_err(|_| state.to_error())?;
							Some(process_index(idx, f.normals.len()))
						}
					};
					//trace!("Face index: \"{}\" => v:{} uv:{:?} norm:{:?}", s, idx_vert, idx_uv, idx_norm);
					Ok(PreIndexInfo::new(idx_vert, idx_uv, idx_norm))
				}
				let fs: Vec<String> = util::parseN_only(&state, 3, &mut args)?;
				let v0 = process_index_info(&state, f, &fs[0])?;
				let v1 = process_index_info(&state, f, &fs[1])?;
				let v2 = process_index_info(&state, f, &fs[2])?;
				f.pre_faces.push(vec3(v0, v1, v2));
			},
			"usemtl" => {
				let m: String = util::parse1_only(&state, &mut args)?;
				f.material = Some(m);
			},
			"s" => {
				// TODO: Figure out what this command does. For now, just ignore it.
			},
			_ => {
				return Err(format!("Unrecognized command `{}` at {}:{}", command, f.rel_path, lno));
			}
		}
	}
	Ok(())
}

/// rel_path = the path of the mtl file relative to the exe.
fn parse_mtl_string(f: &mut ObjFile, path: &Path, rel_path: &Path, s: &str) -> GameResult<()> {
	// Get lines that filter out comments & empty lines
	let li = s.lines()
		.map(|l| l.split("#").next().unwrap_or(""));
	
	let mut current_mat_name = None;
	let mut current_mat = Material::default();
	let mut state = ParseState::new(String::new(), 0, path.to_path_buf(), rel_path.to_path_buf());
	
	for (lno, line) in li.enumerate().map(|(lno, l)| (lno + 1, l)) {
		if line == "" { continue; }
		let mut args = line.split_whitespace().peekable();
		let command = args.next().unwrap_or("");
		state.command = command.to_string();
		state.lno = lno;
		trace!("Mtl State: {:?}", state);
		match command {
			"newmtl" => {
				if let Some(name) = current_mat_name {
					f.materials.insert(name, current_mat.clone());
					current_mat = Material::default();
				}
				current_mat_name = Some(util::parse1_only(&state, &mut args)?);
			},
			"Ns" => {
				let i = util::parse1_only(&state, &mut args)?;
				current_mat.Ns = i;
			}
			"Ka" => { current_mat.Ka = util::parse_vec3_only(&state, &mut args)?; },
			"Kd" => { current_mat.Kd = util::parse_vec3_only(&state, &mut args)?; },
			"Ks" => { current_mat.Ks = util::parse_vec3_only(&state, &mut args)?; },
			"Ke" => { current_mat.Ke = util::parse_vec3_only(&state, &mut args)?; },
			"Ni" => { /* TODO: Figure out what this command is */ },
			"d"  => { current_mat.d = util::parse1_only(&state, &mut args)?; },
			"illum" => { /* TODO: Implement this command */ },
			"map_Ka" => { current_mat.map_Ka = Some(parse_texture_args(&state, &mut args)?); },
			"map_Kd" => { current_mat.map_Kd = Some(parse_texture_args(&state, &mut args)?); },
			_ => {
				return Err(format!("Unrecognized command `{}` at {}:{}", state.command, state.path.display(), state.lno))
			}
		}
	}
	if let Some(name) = current_mat_name {
		f.materials.insert(name, current_mat.clone());
	}
	Ok(())
}

fn parse_texture_args<'a, I>(state: &ParseState, args: &mut Peekable<I>) -> GameResult<(TextureID, TextureOptions)>
		where I: Iterator<Item=&'a str> {
	let a: String = util::parse1(state, args)?;
	if !a.starts_with('-') { // `a` is a texture ID.
		// Ensure that there are no more args
		if args.peek().is_some() { return Err(state.to_error()); }
		
		// Parse `a` as a texture ID
		let id = parse_texture_path(state, &a);
		Ok((id, TextureOptions::default()))
	} else { // `a` is a texture option
		match a.as_str() {
			"-s" => { // "-s u [v] [w]" -- uv scale option
				let u = util::parse1(state, args)?;
				let v = util::parse1_opt(args).unwrap_or(1.0);
				let _ = util::parse1_opt(args).unwrap_or(1.0): Flt; // Ignore the 3D option
				let (id, mut opt) = parse_texture_args(state, args)?; // Recurse on other arguments
				opt.uv_scale = vec2(u, v);
				Ok((id, opt))
			},
			_ => {
				Err(state.to_error() + &format!(": Unknown texture option `{}`", a))
			}
		}
	}
}

fn parse_texture_path(state: &ParseState, id: &str) -> TextureID {
	trace!("id: {}", id);
	let id = Path::new(&id);
	if id.is_absolute() {
		warn!("Absolute path detected in mtl file at {}:{}: {}", state.path.display(), state.lno, id.display());
		id.to_string_lossy().into_owned()
	} else {
		let mut ret = state.rel_path.clone();
		ret.push("..");
		ret.push(&id);
		let ret = util::remove_parents(&ret);
		trace!("ret: {}", ret.display());
		ret.to_string_lossy().into_owned()
	}
}

pub fn load_shader_program(ctx: &Rc<Context>, rel_base: &str) -> GameResult<Program> {
	// TODO: Handle more shader types
	let base = vfs::canonicalize_exe(rel_base);
	
	// Load source of shaders
	let mut vs_src = String::new();
	File::open(base.with_extension("vs"))
		.map_err(|e| format!("Could not open file {}.vs: {}", base.display(), e))?
		.read_to_string(&mut vs_src)
		.map_err(|e| format!("Could not read file {}.vs: {}", base.display(), e))?;
	
	let mut fs_src = String::new();
	File::open(base.with_extension("fs"))
		.map_err(|e| format!("Could not open file {}.fs: {}", base.display(), e))?
		.read_to_string(&mut fs_src)
		.map_err(|e| format!("Could not read file {}.fs: {}", base.display(), e))?;
	
	let prog = Program::from_source(ctx, &vs_src, &fs_src, None)
		.map_err(|e| format!("Could not parse shader {}\n{}", base.display(), e))?;
	
	Ok(prog)
}
