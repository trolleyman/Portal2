use prelude::*;

use std::collections::HashMap;

//use glutin::GlContext;

/// Meshes are identified by their filename
pub type MeshID = String;

pub const MESHID_TEST: &'static str = "res/meshes/test.obj";

pub struct MeshBank {
	//gl: Rc<GlContext>,
	cache: HashMap<MeshID, Mesh>,
}
impl MeshBank {
	pub fn new() -> MeshBank {
		MeshBank {
			cache: HashMap::new(),
		}
	}
	
	pub fn get_mesh(&mut self, id: MeshID) -> GameResult<Mesh> {
		unimplemented!()
	}
	
	pub fn load_mesh(&mut self) {
		unimplemented!()
	}
}

#[derive(Debug)]
pub struct Mesh {
	vertices: Vec<Vec3>,
	indices: Vec<IVec3>,
}
impl Mesh {
	pub fn from_file(filename: &str) -> GameResult<Mesh> {
		use std::io::prelude::*;
		use std::path::PathBuf;
		use std::env::current_exe;
		use std::fs::File;
		
		let mut filepath = current_exe().unwrap_or(PathBuf::from("."));
		filepath.push(filename);
		
		let mut s = String::new();
		let mut f = File::open(filepath);
		match f.and_then(|mut f| f.read_to_string(&mut s)) {
			Err(_) => return GameResult::err(format!("Invalid mesh file (not utf-8): `{}`", filename)),
			_ => {},
		}
		
		Mesh::from_string(&s).map_err(|s| format!("{}: {}", s, filename))
	}
	
	pub fn from_string(s: &str) -> GameResult<Mesh> {
		let mut vertices = vec![];
		let mut indices = vec![];
		
		for (line_number, l) in s.lines().enumerate() {
			// Trim comments
			let l = trim_comments(l);
			// Trim whitespace
			let l = l.trim();
			// Empty line: ignore.
			if l == "" { continue; }
			
			let mut ls = l.split_whitespace();
			let command = ls.next().unwrap();
			if command == "v" {
				// Add to vertices
				let new_vs = [ls.next(), ls.next(), ls.next(), ls.next()]; // Get the next 4 string chunks
				let mut new_vs = new_vs.iter().map(|o| o.and_then(|is| is.parse().ok())); // Try to parse all of them
				let new_vs = take4(&mut new_vs);
				match new_vs {
					[Some(x), Some(y), Some(z), None] => {
						vertices.push(vec3(x, y, z));
					},
					_ => {
						return GameResult::err(format!("Invalid mesh file (invalid command format on line {})", line_number));
					}
				}
			} else if command == "f" {
				// Add to indices
				let new_is = [ls.next(), ls.next(), ls.next(), ls.next()]; // Get the next 4 string chunks
				let mut new_is = new_is.iter().map(|o| o.and_then(|is| is.parse().ok())); // Try to parse all of them
				let new_is = take4(&mut new_is); // Take 4 elements
				match new_is {
					[Some(i0), Some(i1), Some(i2), None] => {
						indices.push(vec3(i0, i1, i2));
					},
					_ => {
						return GameResult::err(format!("Invalid mesh file (invalid command format on line {})", line_number));
					}
				}
			} else {
				// Error on unknown command
				return GameResult::err(format!("Invalid mesh file (unsupported instruction on line {}: `{}`)", line_number, command));
			}
		}
		
		GameResult::ok(Mesh {
			vertices: vertices,
			indices: indices,
		})
	}
}

fn trim_comments(s: &str) -> &str {
	s.split("#").next().unwrap_or("")
}

fn take4_helper<T>(v: Option<Option<T>>) -> Option<T> {
	match v {
		Some(Some(x)) => Some(x),
		_ => None,
	}
}

fn take4<T, U>(it: &mut T) -> [Option<U>; 4] where T: Iterator<Item=Option<U>> {
	[take4_helper(it.next()),
	 take4_helper(it.next()),
	 take4_helper(it.next()),
	 take4_helper(it.next())]
}
