#[allow(unused_imports)]
use prelude::*;
use std::path::{Path, PathBuf};
use std::env::current_exe;

/// Turns a path relative to the executable into absolute terms
pub fn canonicalize_exe<P>(p: P) -> PathBuf where P: AsRef<Path> {
	let p = p.as_ref();
	if p.is_absolute() {
		warn!("canonicalize_exe: Absolute path passed as argument: {}", p.display());
		return p.to_path_buf();
	}
	let mut exe_path = current_exe().unwrap_or(PathBuf::from(".").canonicalize().unwrap_or(PathBuf::from(".")));
	exe_path.pop();
	exe_path.push(p);
	exe_path
}

/// Turns a path that is absolute into relative to the executable
pub fn relative_exe<P>(p: P) -> GameResult<PathBuf> where P: AsRef<Path> {
	let p = p.as_ref();
	let p = p.to_path_buf();
	p.canonicalize().map_err(|e| format!("Could not canonicalize path ({}): {}", e, p.display()))?;
	
	let mut exe_path = current_exe().unwrap_or(PathBuf::from("."));
	exe_path.pop();
	match p.strip_prefix(&exe_path) {
		Ok(ret) => Ok(ret.to_path_buf()),
		Err(e) => Err(format!("Strip prefix didn't work on path ({}): {} (executable path: {})", e, p.display(), exe_path.display()))
	}
}
