use std::{cmp::PartialEq, fmt, hash::Hash};

use super::PropertyCardKind;
use crate::cards::{Card, Colored, Play};
use crate::color::{colored_text, CardColor, MultiColor};
use crate::game::Turn;
use crate::player::Assets;

/// Represents a property wild card.
#[derive(Debug, Hash, Eq, PartialEq)]
pub struct PropertyWildCard {
	/// Represents the money the card will amount to when paid as rent.
	pub value: u8,

	/// Holds all the colors on the card.
	pub available_colors: MultiColor,

	/// Represents the current color of the card.
	pub selected_color: Option<CardColor>,
}

impl PropertyWildCard {
	pub fn new(value: u8, colors: MultiColor) -> Self {
		Self {
			value,
			available_colors: colors,
			selected_color: None,
		}
	}
}

impl Card for PropertyWildCard {
	fn value(&self) -> u8 {
		self.value
	}
}

impl Play for PropertyWildCard {
	fn is_playable(&self, _: &Assets) -> bool {
		true
	}

	fn play(self, turn: &mut Turn) {
		turn.assets.add_property(self.into());
	}
}

impl Colored for PropertyWildCard {
	fn set_color(&mut self, color: CardColor) {
		self.selected_color = Some(color);
	}

	fn colors(&self) -> Vec<CardColor> {
		Vec::from(self.available_colors)
	}
}

impl std::convert::From<PropertyWildCard> for PropertyCardKind {
	fn from(property_wild_card: PropertyWildCard) -> Self {
		Self::Wild(property_wild_card)
	}
}

impl fmt::Display for PropertyWildCard {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let card_name = "PropertyWildCard";

		let formatted_text = match self.selected_color {
			Some(color) => colored_text(card_name, color),
			None => String::from(card_name),
		};

		write!(f, "{} {}", formatted_text, self.available_colors)
	}
}
