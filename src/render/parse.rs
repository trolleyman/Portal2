use prelude::*;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::rc::Rc;

use render::{TextureID, Material};

use glium::Program;
use glium::backend::Context;

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
		
		// Validate pre_faces, so that we know all indices are in bounds
		f.validate()?;
				
		// Calculate faces from pre_faces
		f.calculate_faces();
		
		Ok(f)
	}
	
	/// Calculates the faces from pre_faces
	fn calculate_faces(&mut self) {
		trace!("Calculating faces...");
		// Reserve space for faces
		let add = self.pre_faces.len() as isize - self.faces.len() as isize;
		self.faces.reserve( if add < 0 { 0 } else { add as usize } );
		
		// Add default uv (0.0, 0.0)
		self.uvs.push(vec2(0.0, 0.0));
		let def_uv_idx = (self.uvs.len() - 1) as Idx;
		
		// Process faces
		for f in self.pre_faces.iter() {
			// Calculate normals
			let normals = if f.x.norm.is_none() || f.y.norm.is_none() || f.z.norm.is_none() {
				let v0 = self.vertices[f.x.vert as usize];
				let v1 = self.vertices[f.y.vert as usize];
				let v2 = self.vertices[f.z.vert as usize];
				let n = (v1 - v0).cross(v2 - v0).normalize();
				self.normals.push(n);
				let i = (self.normals.len() - 1) as Idx;
				vec3(f.x.norm.unwrap_or(i), f.y.norm.unwrap_or(i), f.z.norm.unwrap_or(i))
			} else {
				vec3(f.x.norm.unwrap()    , f.y.norm.unwrap()    , f.z.norm.unwrap()    )
			};
			let face = vec3(
				IndexInfo::new(f.x.vert, f.x.uv.unwrap_or(def_uv_idx), normals.x),
				IndexInfo::new(f.y.vert, f.y.uv.unwrap_or(def_uv_idx), normals.y),
				IndexInfo::new(f.z.vert, f.z.uv.unwrap_or(def_uv_idx), normals.z));
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
struct ParseState {
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
		.map(|l| l.split("#").next().unwrap_or(""))
		.filter(|&l| l != "");
	
	let mut state = ParseState::new(String::new(), 0, f.path.clone(), PathBuf::from(&f.rel_path));
	
	for (lno, line) in li.enumerate().map(|(lno, l)| (lno + 1, l)) {
		let mut args = line.split_whitespace();
		let command = args.next().unwrap_or("");
		state.command = command.to_string();
		state.lno = lno;
		trace!("Obj State: {:?}", state);
		match command {
			"mtllib" => {
				let mtl_rel_path = args.next()
					.ok_or_else(|| state.to_error())?;
				
				let mtl_rel_exe_path = remove_parents(&Path::new(&f.rel_path).join("..").join(&mtl_rel_path));
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
				f.name = Some(parse1(&state, &mut args)?);
			},
			"v" => {
				let v = parse_vec3(&state, &mut args)?;
				f.vertices.push(v);
			},
			"vt" => {
				let v = parseN(&state, 2, &mut args)?;
				f.uvs.push(vec2(v[0], v[1]));
			},
			"vn" => {
				let v = parse_vec3(&state, &mut args)?;
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
				let fs: Vec<String> = parseN(&state, 3, &mut args)?;
				let v0 = process_index_info(&state, f, &fs[0])?;
				let v1 = process_index_info(&state, f, &fs[1])?;
				let v2 = process_index_info(&state, f, &fs[2])?;
				f.pre_faces.push(vec3(v0, v1, v2));
			},
			"usemtl" => {
				let m: String = parse1(&state, &mut args)?;
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
		.map(|l| l.split("#").next().unwrap_or(""))
		.filter(|&l| l != "");
	
	let mut current_mat_name = None;
	let mut current_mat = Material::default();
	let mut state = ParseState::new(String::new(), 0, path.to_path_buf(), rel_path.to_path_buf());
	
	for (lno, line) in li.enumerate().map(|(lno, l)| (lno + 1, l)) {
		let mut args = line.split_whitespace();
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
				current_mat_name = Some(parse1(&state, &mut args)?);
			},
			"Ns" => {
				let i = parse1(&state, &mut args)?;
				current_mat.Ns = i;
			}
			"Ka" => { current_mat.Ka = parse_vec3(&state, &mut args)?; },
			"Kd" => { current_mat.Kd = parse_vec3(&state, &mut args)?; },
			"Ks" => { current_mat.Ks = parse_vec3(&state, &mut args)?; },
			"Ke" => { current_mat.Ke = parse_vec3(&state, &mut args)?; },
			"Ni" => { /* TODO: Figure out what this command is */ },
			"d"  => { current_mat.d = parse1(&state, &mut args)?; },
			"illum" => { /* TODO: Implement this command */ },
			"map_Ka" => {
				let id: String = parse1(&state, &mut args)?;
				let id = parse_texture_path(&state, &rel_path, &id);
				current_mat.map_Ka = Some(id);
			},
			"map_Kd" => {
				let id: String = parse1(&state, &mut args)?;
				let id = parse_texture_path(&state, &rel_path, &id);
				current_mat.map_Kd = Some(id);
			},
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

fn parse_vec3<'a, I>(st: &ParseState, it: &mut I) -> GameResult<Vec3>
		where I: Iterator<Item=&'a str> {
	let v = parseN(st, 3, it)?;
	let v = vec3(v[0], v[1], v[2]);
	Ok(v)
}

fn parse1<'a, F: FromStr, I>(st: &ParseState, it: &mut I) -> GameResult<F>
		where I: Iterator<Item=&'a str> {
	let a = it.next().ok_or_else(|| st.to_error())?;
	a.parse().map_err(|_| st.to_error())
}

fn parseN<'a, F: FromStr, I>(st: &ParseState, n: usize, it: &mut I) -> GameResult<Vec<F>>
		where I: Iterator<Item=&'a str> {
	let mut ret = Vec::with_capacity(n);
	for _ in 0..n {
		ret.push(parse1(st, it)?);
	}
	Ok(ret)
}

// Removes all unnecesary parents in a path
fn remove_parents(p: &Path) -> PathBuf {
	let mut ret = PathBuf::new();
	let mut n = 0;
	for c in p.components() {
		let c_str = c.as_os_str();
		if c_str == ".." && n == 0 {
			ret.push(c_str);
		} else if c_str == ".." {
			ret.pop();
			n -= 1;
		} else {
			ret.push(c_str);
			n += 1;
		}
	}
	ret
}

fn parse_texture_path(state: &ParseState, base_path: &Path, id: &str) -> TextureID {
	trace!("id: {}", id);
	let id = Path::new(&id);
	if id.is_absolute() {
		warn!("Absolute path detected in mtl file at {}:{}: {}", state.path.display(), state.lno, id.display());
		id.to_string_lossy().into_owned()
	} else {
		let mut ret = base_path.to_path_buf();
		ret.push("..");
		ret.push(&id);
		let ret = remove_parents(&ret);
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

#[cfg(test)]
mod test {
	#[test]
	fn test_remove_parents() {
		use std::path::Path;
		macro_rules! trp {
			($input:expr, $expect:expr) => ({
				let input = Path::new($input);
				let expected = Path::new($expect);
				let ret = super::remove_parents(&input);
				assert_eq!(ret, expected);
			})
		}
		
		trp!("thing/other/../no_wait/", "thing/no_wait/");
		trp!("../thing/../other/", "../other/");
		trp!("../../../thing/thing2/../o", "../../../thing/o");
	}
}