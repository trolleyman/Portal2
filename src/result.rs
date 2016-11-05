use std::convert::{Into, From};
use std::string::ToString;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ops::Carrier;

pub struct GameResult<T>(Result<T, String>);

impl<T> GameResult<T> {
	pub fn ok(v: T) -> GameResult<T> {
		GameResult(Ok(v))
	}
	pub fn err<U>(v: U) -> GameResult<T> where U: ToString {
		GameResult(Err(v.to_string()))
	}
}

impl<T, U> From<Result<T, U>> for GameResult<T> where U: ToString {
	 fn from(r: Result<T, U>) -> GameResult<T> {
		 GameResult(r.map_err(|e| e.to_string()))
	 }
}
impl<T> Into<Result<T, String>> for GameResult<T> {
	fn into(self) -> Result<T, String> {
		self.0
	} 
}

impl<T> Carrier for GameResult<T> {
	type Success = T;
	type Error = String;
	fn from_success(t: T) -> GameResult<T> {
		GameResult(Ok(t))
	}
	fn from_error(s: String) -> GameResult<T> {
		GameResult(Err(s))
	}
	fn translate<U>(self) -> U where U: Carrier<Success=T, Error=String> {
		match self.0 {
			Ok(v) => U::from_success(v),
			Err(e) => U::from_error(e),
		}
	}
}
