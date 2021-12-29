use crossterm::style::Stylize;
use std::fmt;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub enum Color {
	Blue,
	Green,
	Magenta,
	Red,
	Yellow,
	LightBlue,
	LightGreen,
	LightMagenta,
	LightRed,
	LightYellow,
	White,
}

pub const COLORS: [Color; 10] = [
	Color::Blue,
	Color::Green,
	Color::Magenta,
	Color::Red,
	Color::Yellow,
	Color::LightBlue,
	Color::LightGreen,
	Color::LightMagenta,
	Color::LightRed,
	Color::LightYellow,
];

pub const BLOCK: &'static str = "⬤";

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum MultiColor {
	Two(Color, Color),
	All,
}

impl MultiColor {
	pub fn colors(&self) -> Vec<Color> {
		match self {
			MultiColor::Two(c, d) => vec![*c, *d],
			MultiColor::All => COLORS.to_vec(),
		}
	}
}

impl fmt::Display for MultiColor {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.colors()
			.iter()
			.map(|&color| colored_text(BLOCK, color))
			.collect::<Vec<String>>()
			.join(" ")
			.fmt(f)
	}
}

pub fn colored_text(text: &'static str, color: Color) -> String {
	let colorizer = match color {
		Color::Blue => Stylize::dark_blue,
		Color::Green => Stylize::dark_green,
		Color::Magenta => Stylize::dark_magenta,
		Color::Red => Stylize::dark_red,
		Color::Yellow => Stylize::dark_yellow,
		Color::LightBlue => Stylize::blue,
		Color::LightGreen => Stylize::green,
		Color::LightMagenta => Stylize::magenta,
		Color::LightRed => Stylize::red,
		Color::LightYellow => Stylize::yellow,
		Color::White => Stylize::white,
	};

	return colorizer(text).to_string();
}
