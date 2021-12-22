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
	use Color::*;

	let colorizer = match color {
		Blue => Stylize::dark_blue,
		Green => Stylize::dark_green,
		Magenta => Stylize::dark_magenta,
		Red => Stylize::dark_red,
		Yellow => Stylize::dark_yellow,
		LightBlue => Stylize::blue,
		LightGreen => Stylize::green,
		LightMagenta => Stylize::magenta,
		LightRed => Stylize::red,
		LightYellow => Stylize::yellow,
	};

	return colorizer(text).to_string();
}

// XXX: Implement `Iterator`?
