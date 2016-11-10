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
pub fn relative_exe<P>(p: P) -> PathBuf where P: AsRef<Path> {
	let p = p.as_ref();
	let p_clone = p.to_path_buf();
	let p = match p.canonicalize() {
		Ok(p) => p,
		Err(e) => {
			warn!("relative_exe: Could not canonicalize path ({}): {}", e, p.display());
			p_clone
		}
	};
	let mut exe_path = current_exe().unwrap_or(PathBuf::from("."));
	exe_path.pop();
	match p.strip_prefix(&exe_path) {
		Ok(ret) => return ret.to_path_buf(),
		Err(e) => {
			warn!("relative_exe: Strip prefix didn't work on path ({}): {} (executable path: {})", e, p.display(), exe_path.display());
		},
	}
	p
}
