use std::convert::From;
use std::fmt::Display;
use std::num::ParseIntError;

pub enum Failed {
	NotANumber,
	InvalidIndex(usize),
	NotPlayable(String),
}

impl Display for Failed {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::NotANumber => write!(f, "Not a valid number o_0"),
			Self::InvalidIndex(i) => write!(f, "{} is out of bounds :<", i),
			Self::NotPlayable(reason) => reason.fmt(f),
		}
	}
}

pub struct NotPlayable(pub String);

impl From<ParseIntError> for Failed {
	fn from(_: ParseIntError) -> Self {
		Self::NotANumber
	}
}

impl From<NotPlayable> for Failed {
	fn from(NotPlayable(reason): NotPlayable) -> Self {
		Self::NotPlayable(reason)
	}
}
