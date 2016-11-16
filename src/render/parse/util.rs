use prelude::*;

use std::path::{Path, PathBuf};
use std::str::FromStr;

use super::ParseState;

// The `*_only` variants of the `parse*` functions only parse the specified amount,
// and error when there are more arguments

pub fn parse_vec3_only<'a, I>(st: &ParseState, it: &mut I) -> GameResult<Vec3>
		where I: Iterator<Item=&'a str> {
	let v = parseN_only(st, 3, it)?;
	let v = vec3(v[0], v[1], v[2]);
	Ok(v)
}

pub fn parse1_only<'a, F: FromStr, I>(st: &ParseState, it: &mut I) -> GameResult<F>
		where I: Iterator<Item=&'a str> {
	let a = it.next().ok_or_else(|| st.to_error())?;
	if it.next().is_some() { return Err(st.to_error()); }
	a.parse().map_err(|_| st.to_error())
}

// rename parse1
pub fn parseN_it<'a, F: FromStr, I>(st: &ParseState, it: &mut I) -> GameResult<F>
		where I: Iterator<Item=&'a str> {
	let a = it.next().ok_or_else(|| st.to_error())?;
	a.parse().map_err(|_| st.to_error())
}

pub fn parseN_only<'a, F: FromStr, I>(st: &ParseState, n: usize, it: &mut I) -> GameResult<Vec<F>>
		where I: Iterator<Item=&'a str> {
	let mut ret = Vec::with_capacity(n);
	for _ in 0..n {
		ret.push(parseN_it(st, it)?);
	}
	Ok(ret)
}

// Removes all unnecesary parents in a path
pub fn remove_parents(p: &Path) -> PathBuf {
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