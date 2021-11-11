use std::fmt;

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
		match self {
			Color::Black => "Black",
			Color::Blue => "Blue",
			Color::Brown => "Brown",
			Color::Green => "Green",
			Color::LightBlue => "Light Blue",
			Color::LightGreen => "Light Green",
			Color::Orange => "Orange",
			Color::Pink => "Pink",
			Color::Red => "Red",
			Color::Yellow => "Yellow",
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
