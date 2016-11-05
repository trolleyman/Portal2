use prelude::*;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use render::texture::TextureID;

use vfs;

#[derive(Debug)]
pub struct Material {
	/// Ambient colour
	pub Ka: Vec3,
	/// Difuse colour
	pub Kd: Vec3,
	/// Specular colour
	pub Ks: Vec3,
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
			Ns: 10.0,
			d: 1.0,
			map_Ka: None,
			map_Kd: None,
			map_Ks: None,
			map_Ns: None,
			map_d: None,
			bump: None,
			disp: None,
		}
	}
}

#[derive(Debug)]
pub struct ObjFile {
	/// Filepath (relative to current exe pos) e.g. "res/mesh/test.obj"
	pub path: String,
	/// Name of the object
	pub name: Option<String>,
	/// Materials read
	pub materials: HashMap<String, Material>,
}

pub fn parse_obj_file(filename: String) -> GameResult<ObjFile> {
	ObjParser::new(filename).parse()
}

struct ObjParser {
	filename: String,
	path: PathBuf,
	name: Option<String>,
	materials: HashMap<String, Material>,
}
impl ObjParser {
	fn new(filename: String) -> ObjParser {
		ObjParser {
			filename: filename.clone(),
			path: vfs::relative_to_exe(&filename),
			name: None,
			materials: HashMap::new(),
		}
	}
	
	fn parse(self) -> GameResult<ObjFile> {
		let mut s = String::new();
		File::open(&self.path)
			.map_err(|e| format!("Invalid obj file ({}): {}", e, &self.filename))?
			.read_to_string(&mut s)
			.map_err(|e| format!("Invalid obj file ({}): {}", e, &self.filename))?;
		
		self.parse_string(s)
	}
	
	fn parse_string(mut self, s: String) -> GameResult<ObjFile> {
		fn cfmt_error(command: &str, lno: usize, filename: &str) -> String {
			format!("Invalid command format `{}` at location {}:{}", command, lno, filename)
		}
		
		// Get an iterator that ignores comments and empty lines
		let mut li = s.lines()
			.map(|l| l.split("#").next().unwrap_or(""))
			.filter(|&l| l != "");
		
		for (lno, line) in li.enumerate().map(|(lno, l)| (lno + 1, l)) {
			let mut args = line.split_whitespace();
			let command = args.next().unwrap(); // Should be fine as there should be no empty lines
			match command {
				"mtllib" => {
					let mtl_filename = args.next()
						.ok_or_else(|| cfmt_error(command, lno, &self.filename))?;
					
					// Get the path of the mtl lib, as it is relative to the current file.
					let mut mtl_path = self.path.clone();
					mtl_path.pop();
					mtl_path.push(mtl_filename);
					// Load the mtl file
					let mut mtl_s = String::new();
					File::open(&mtl_path)
						.and_then(|mut f| f.read_to_string(&mut mtl_s))
						.map_err(|e| format!("Invalid mtl file ({}): {}", e, mtl_path.display()))?;
					self.parse_mtl_file(&mtl_s);
				},
				_ => {
					return Err(format!("Unrecognized command `{}` at location {}:{}", command, lno, self.filename));
				}
			}
		}
		
		Ok(ObjFile {
			path: self.filename,
			name: self.name,
			materials: unimplemented!(),
		})
	}
	
	fn parse_mtl_file(&mut self, s: &str) {
		let mut li = s.lines()
			.map(|l| l.split("#").next().unwrap_or(""))
			.filter(|&l| l != "");
		
		for (lno, line) in li.enumerate() {
			
		}
		unimplemented!();
	}
}
