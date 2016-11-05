use std::string::ToString;

pub type GameResult<T> = Result<T, String>;

pub trait IntoGameResult<T> {
	fn into_game_result(self) -> GameResult<T>;
}

impl<T, U> IntoGameResult<T> for Result<T, U> where U: ToString {
	fn into_game_result(self) -> GameResult<T> {
		self.map_err(|e| e.to_string())
	}
}
