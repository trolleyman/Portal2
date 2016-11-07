use prelude::*;
use std::path::PathBuf;
use std::env::current_exe;

/// Makes a path relative to the executable (or if it can't find it, the current directory)
pub fn relative_to_exe(p: &str) -> PathBuf {
	let mut filepath = current_exe().unwrap_or(PathBuf::from("."));
	filepath.pop();
	filepath.push(p);
	filepath
}
