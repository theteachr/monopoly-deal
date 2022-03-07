use std::convert::From;
use std::fmt::Display;
use std::num::ParseIntError;

pub enum Failed {
	InvalidIndex,
	NotPlayable(String),
}

impl Display for Failed {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::InvalidIndex => write!(f, "Invalid index :o"),
			Self::NotPlayable(reason) => reason.fmt(f),
		}
	}
}

pub struct NotPlayable(pub String);

impl From<ParseIntError> for Failed {
	fn from(_: ParseIntError) -> Self {
		Self::InvalidIndex
	}
}

impl From<NotPlayable> for Failed {
	fn from(NotPlayable(reason): NotPlayable) -> Self {
		Self::NotPlayable(reason)
	}
}
