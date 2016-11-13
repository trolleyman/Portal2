use prelude::*;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use render::texture::TextureID;
use render::Material;

use vfs;

/// Face is a series of 3 points representing a triangle
/// 
/// f 1/2/3 4/5/6 7/8/9 == vec3(vec3(1,2,3), vec3(4,5,6), vec3(7,8,9))
pub type Face = Vector3<Vector3<Idx>>;

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
	pub faces: Vec<Face>,
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
		};
		
		parse_file(&mut f)?;
		
		// Validate file
		f.validate()?;
		
		Ok(f)
	}
	
	/// Validate the file
	fn validate(&self) -> GameResult<()> {
		// Check if material is one of the materials read
		if let Some(ref m) = self.material {
			if self.materials.get(m).is_none() {
				return Err(format!("Invalid obj file ({}): Unknown material `{}`", self.rel_path, m));
			}
		}
		
		// Check if all faces are valid
		for (i, f) in self.faces.iter().enumerate() {
			fn check_face_vertex(i: usize, o: &ObjFile, f: Vector3<Idx>) -> GameResult<()> {
				if f.x as usize >= o.vertices.len() {
					return Err(format!("Invalid obj file ({}): Invalid face (index {}) vertex index: {}", &o.rel_path, i, f.x));
				} else if f.y as usize >= o.uvs.len() {
					return Err(format!("Invalid obj file ({}): Invalid face (index {}) uv index: {}", &o.rel_path, i, f.y));
				} else if f.z as usize >= o.normals.len() {
					return Err(format!("Invalid obj file ({}): Invalid face (index {}) normal index: {}", &o.rel_path, i, f.z));
				}
				Ok(())
			}
			check_face_vertex(i, self, f.x)?;
			check_face_vertex(i, self, f.y)?;
			check_face_vertex(i, self, f.z)?;
		}
		if self.material.is_none() {
			warn!("Object file loaded without a material: {}", &self.rel_path);
		}
		Ok(())
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

#[derive(Debug)]
struct ParseState {
	command: String,
	lno: usize,
	path: PathBuf,
}
impl ParseState {
	pub fn new(command: String, lno: usize, path: PathBuf) -> ParseState {
		ParseState {
			command: command,
			lno: lno,
			path: path,
		}
	}
	pub fn to_error(&self) -> String {
		format!("Invalid command format `{}` at location {}:{}", self.command, self.lno, self.path.display())
	}
}

fn parse_string(f: &mut ObjFile, s: String) -> GameResult<()> {
	// Get an iterator that ignores comments and empty lines
	let li = s.lines()
		.map(|l| l.split("#").next().unwrap_or(""))
		.filter(|&l| l != "");
	
	let mut state = ParseState::new(String::new(), 0, f.path.clone());
	
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
				
				// Get the path of the mtl lib, as it is relative to the current file.
				let mut mtl_path = f.path.clone();
				mtl_path.pop();
				mtl_path.push(mtl_rel_path);
				// Load the mtl file
				let mut mtl_s = String::new();
				File::open(&mtl_path)
					.and_then(|mut f| f.read_to_string(&mut mtl_s))
					.map_err(|e| format!("Invalid mtl file ({}): {}", e, mtl_path.display()))?;
				parse_mtl_string(f, &mtl_path, &mtl_s)?;
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
				fn process_face_vertex(state: &ParseState, f: &ObjFile, s: &str) -> GameResult<Vector3<Idx>> {
					let mut f_it = s.split("/");
					let f_indices: Vec<isize> = parseN(&state, 3, &mut f_it)?;
					let i0 = process_index(f_indices[0], f.vertices.len());
					let i1 = process_index(f_indices[1], f.uvs.len());
					let i2 = process_index(f_indices[2], f.normals.len());
					Ok(vec3(i0, i1, i2))
				}
				let fs: Vec<String> = parseN(&state, 3, &mut args)?;
				let v0 = process_face_vertex(&state, f, &fs[0])?;
				let v1 = process_face_vertex(&state, f, &fs[1])?;
				let v2 = process_face_vertex(&state, f, &fs[2])?;
				f.faces.push(vec3(v0, v1, v2));
			},
			"usemtl" => {
				let m: String = parse1(&state, &mut args)?;
				f.material = Some(m);
			},
			"s" => {
				// TODO: Figure out what this command does. For now, just ignore it.
			},
			_ => {
				return Err(format!("Unrecognized command `{}` at location {}:{}", command, lno, f.rel_path));
			}
		}
	}
	Ok(())
}

fn parse_mtl_string(f: &mut ObjFile, path: &Path, s: &str) -> GameResult<()> {
	// Get lines that filter out comments & empty lines
	let li = s.lines()
		.map(|l| l.split("#").next().unwrap_or(""))
		.filter(|&l| l != "");
	
	let mut current_mat_name = None;
	let mut current_mat = Material::default();
	let mut state = ParseState::new(String::new(), 0, path.to_path_buf());
	
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
			"illum" => { /* TODO: Implement this command */ }
			"map_Kd" => {
				let s: String = parse1(&state, &mut args)?;
				match parse_texture_id(&state, &s) {
					Ok(id) => { current_mat.map_Kd = Some(id); },
					Err(e) => { warn!("Could not load texture ({}) at location {}:{}: {}", s, state.path.display(), state.lno, e); }
				}
			},
			_ => {
				return Err(format!("Unrecognized command `{}` at location {}:{}", state.command, state.lno, state.path.display()))
			}
		}
	}
	if let Some(name) = current_mat_name {
		f.materials.insert(name, current_mat.clone());
	}
	Ok(())
}

fn parse_texture_id(st: &ParseState, s: &str) -> GameResult<TextureID> {
	// s == '../img/whatever.png'
	if !PathBuf::from(s).is_relative() {
		let rel_path = vfs::relative_exe(s)?;
		Ok(rel_path.to_string_lossy().into_owned())
	} else {
		// p == 'C:/....../res/mesh/thing.obj/../../img/whatever.png'
		let p = st.path.join("..").join(s);
		// p == 'C:/....../res/img/whatever.png'
		p.canonicalize().map_err(|e| format!("Could not canonicalize path ({}): {}", e, p.display()))?;
		// rel_path == 'res/img/whatever.png' (hopefully)
		let rel_path = vfs::relative_exe(&p)?;
		Ok(rel_path.to_string_lossy().into_owned())
	}
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
