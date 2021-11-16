use std::fmt;

// TODO use terminal friendly colors

#[derive(Copy, Clone)]
pub enum Color {
	Black,
	Blue,
	Brown,
	Green,
	LightBlue,
	LightGreen,
	Orange,
	Pink,
	Red,
	Yellow,
}

impl Color {
	fn to_string(&self) -> &'static str {
		use Color::*;

		match self {
			     Black => "Black",
			      Blue => "Blue",
			     Brown => "Brown",
			     Green => "Green",
			 LightBlue => "Light Blue",
			LightGreen => "Light Green",
			    Orange => "Orange",
			      Pink => "Pink",
			       Red => "Red",
			    Yellow => "Yellow",
		}
	}
}

impl fmt::Debug for Color {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Color {}", self.to_string())
	}
}

impl fmt::Display for Color {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.to_string())
	}
}
