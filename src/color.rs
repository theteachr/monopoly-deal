use std::{collections::HashSet, fmt, hash::Hash};

/// Represents the color of a property card.
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

/// Represents a color as a three tuple holding R G B values.
type Color = (u8, u8, u8);

/// Store RGB values of all colors.
const RGB_TRIPLES: [Color; 10] = [
	(000, 000, 000), // Black
	(010, 147, 150), // Blue
	(150, 075, 000), // Brown
	(083, 221, 108), // Green
	(214, 122, 177), // Magenta
	(255, 120, 079), // Orange
	(232, 049, 081), // Red
	(132, 218, 235), // SkyBlue
	(148, 210, 189), // Turquoise
	(244, 233, 000), // Yellow
];

/// Store all the color variants in an array.
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

/// Represents the color type of a colored card.
///
/// ## Examples
///
/// A `RentCard` with `Magenta` and `Yellow` on it.
/// A `PropertyWildCard`.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum MultiColor {
	/// Represents the colors of a dual colored card.
	Two(CardColor, CardColor),

	/// Represents a wild colored card, that can be played as any color.
	All,
}

impl MultiColor {
	pub fn get(&self) -> HashSet<CardColor> {
		match self {
			Self::Two(c, d) => vec![*c, *d].into_iter().collect(),
			Self::All => COLORS.to_vec().into_iter().collect(),
		}
	}
}

impl CardColor {
	/// Returns the R G B value corresponding to the `CardColor`.
	pub fn to_rgb(self) -> Color {
		RGB_TRIPLES[self as usize]
	}
}

impl fmt::Display for MultiColor {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.get()
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

/// Returns `text` colored in `color` with terminal escape sequences.
pub fn colored_text(text: &'static str, color: CardColor) -> String {
	let (r, g, b) = color.to_rgb();

	return format!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, text);
}
