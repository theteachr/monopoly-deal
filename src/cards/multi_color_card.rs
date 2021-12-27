use crate::color::{colored_text, Color, MultiColor};
use std::fmt;

#[derive(Debug, Eq, Copy, Clone, PartialEq, Hash)]
struct MultiColorCard {
	text: &'static str,
	value: u8,
	available_colors: MultiColor,
	selected_color: Option<Color>,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct RentCard {
	card: MultiColorCard,
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
pub struct PropertyWildCard {
	card: MultiColorCard,
}

impl RentCard {
	pub fn new(value: u8, colors: MultiColor) -> Self {
		Self {
			card: MultiColorCard::new("RentCard", value, colors),
		}
	}
}

impl PropertyWildCard {
	pub fn new(value: u8, colors: MultiColor) -> Self {
		Self {
			card: MultiColorCard::new("PropertyWildCard", value, colors),
		}
	}
}

impl MultiColorCard {
	pub fn new(text: &'static str, value: u8, colors: MultiColor) -> Self {
		Self {
			text: text,
			value,
			available_colors: colors,
			selected_color: None,
		}
	}
}

impl fmt::Display for PropertyWildCard {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.card.fmt(f)
	}
}

impl fmt::Display for RentCard {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.card.fmt(f)
	}
}

impl fmt::Display for MultiColorCard {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let color = match self.selected_color {
			Some(c) => c,
			None => Color::White,
		};

		write!(
			f,
			"{} {}",
			colored_text(self.text, color),
			self.available_colors,
		)
	}
}
