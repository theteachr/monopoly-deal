use crossterm::style::Stylize;

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
	};

	return colorizer(text).to_string();
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
