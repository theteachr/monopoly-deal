use std::fmt;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum CardColor {
	Black,
	Blue,
	Brown,
	Green,
	Magenta,
	Orange,
	Red,
	SkyBlue,
	Turquoise,
	Yellow,
}

type Color = (u8, u8, u8);

const RGB_TRIPLES: [Color; 10] = [
	(013, 024, 033), // Black
	(010, 147, 150), // Blue
	(155, 034, 038), // Brown
	(083, 221, 108), // Green
	(214, 122, 177), // Magenta
	(255, 120, 079), // Orange
	(232, 049, 081), // Red
	(132, 218, 235), // SkyBlue
	(148, 210, 189), // Turquoise
	(244, 157, 055), // Yellow
];

pub const COLORS: [CardColor; 10] = [
	CardColor::Black,
	CardColor::Blue,
	CardColor::Brown,
	CardColor::Green,
	CardColor::Magenta,
	CardColor::Orange,
	CardColor::Red,
	CardColor::SkyBlue,
	CardColor::Turquoise,
	CardColor::Yellow,
];

pub const BLOCK: &'static str = "⬤";

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum MultiColor {
	Two(CardColor, CardColor),
	All,
}

impl CardColor {
	pub fn to_rgb(self) -> Color {
		RGB_TRIPLES[self as usize]
	}
}

impl MultiColor {
	pub fn colors(&self) -> Vec<CardColor> {
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

impl fmt::Display for CardColor {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		colored_text(BLOCK, *self).fmt(f)
	}
}

pub fn colored_text(text: &'static str, color: CardColor) -> String {
	let (r, g, b) = color.to_rgb();

	return format!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, text);
}
