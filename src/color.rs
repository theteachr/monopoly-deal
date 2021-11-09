use std::fmt;

#[derive(Copy, Clone)]
pub enum Color {
	Brown,
	Blue,
	Green,
	LightBlue,
	Orange,
	Pink,
	Black,
	Red,
	LightGreen,
	Yellow,
}

fn to_str(variant: Color) -> &'static str {
	match variant {
		Color::Brown => "Brown",
		Color::Blue => "Blue",
		Color::Green => "Green",
		Color::LightBlue => "Light Blue",
		Color::Orange => "Orange",
		Color::Pink => "Pink",
		Color::Black => "Black",
		Color::Red => "Red",
		Color::LightGreen => "Light Green",
		Color::Yellow => "Yellow",
	}
}

impl fmt::Debug for Color {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Color {}", to_str(*self))
	}
}

impl fmt::Display for Color {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", to_str(*self))
	}
}
