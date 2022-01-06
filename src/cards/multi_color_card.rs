use crate::color::{colored_text, CardColor, MultiColor};
use std::fmt;

#[derive(Debug, Eq, Copy, Clone, PartialEq, Hash)]
pub enum MultiColorCardKind {
	RentCard,
	PropertyWildCard,
}

#[derive(Debug, Eq, Copy, Clone, PartialEq, Hash)]
pub struct MultiColorCard {
	kind: MultiColorCardKind,
	value: u8,
	available_colors: MultiColor,
	selected_color: Option<CardColor>,
}

impl MultiColorCard {
	pub fn new(kind: MultiColorCardKind, value: u8, colors: MultiColor) -> Self {
		Self {
			kind,
			value,
			available_colors: colors,
			selected_color: None,
		}
	}

	pub fn set_color(&mut self, color: CardColor) {
		self.selected_color = Some(color);
	}

	pub fn colors(&self) -> Vec<CardColor> {
		self.available_colors.colors()
	}
}

impl MultiColorCardKind {
	pub fn to_static_str(&self) -> &'static str {
		match self {
			Self::RentCard => "RentCard",
			Self::PropertyWildCard => "PropertyWildCard",
		}
	}
}

impl fmt::Display for MultiColorCard {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let card_name = self.kind.to_static_str();

		let text = match self.selected_color {
			Some(color) => colored_text(card_name, color),
			None => String::from(card_name),
		};

		write!(f, "{} {} ", text, self.available_colors)
	}
}
