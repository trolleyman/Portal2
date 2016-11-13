#[allow(unused_imports)]
use prelude::*;
use std::path::{Path, PathBuf};
use std::env::current_exe;

fn exe_dir() -> PathBuf {
	current_exe().ok()
		.and_then(|p| p.parent().map(|p| p.to_path_buf()))
		.and_then(|p| p.canonicalize().ok())
		.unwrap_or(PathBuf::from("."))
}

fn join_exe_dir(p: &Path) -> PathBuf {
	let mut dir = exe_dir();
	for c in p.components() {
		dir.push(c.as_os_str());
	}
	dir
}

/// Turns a path relative to the executable into absolute terms
pub fn canonicalize_exe<P>(p: P) -> PathBuf where P: AsRef<Path> {
	let p = p.as_ref();
	if p.is_absolute() {
		warn!("canonicalize_exe: Absolute path passed as argument: {}", p.display());
		return p.to_path_buf();
	}
	trace!("exe_dir: {}", exe_dir().display());
	let abs_path = join_exe_dir(p);
	match abs_path.canonicalize() {
		Ok(p) => p,
		Err(e) => {
			trace!("Could not canonicalize path ({}): {}", e, abs_path.display());
			abs_path
		},
	}
}

/// Turns a path that is absolute into relative to the executable
pub fn relative_exe<P>(p: P) -> GameResult<PathBuf> where P: AsRef<Path> {
	let p = p.as_ref();
	let p = p.to_path_buf();
	p.canonicalize().map_err(|e| format!("Could not canonicalize path ({}): {}", e, p.display()))?;
	
	let exe_dir = exe_dir();
	match p.strip_prefix(&exe_dir) {
		Ok(ret) => Ok(ret.to_path_buf()),
		Err(e) => Err(format!("Strip prefix didn't work on path ({}): {} (executable dir: {})", e, p.display(), exe_dir.display()))
	}
}
