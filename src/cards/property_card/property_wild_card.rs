use std::{cmp::PartialEq, fmt, hash::Hash};

use super::PropertyCardKind;
use crate::cards::Colored;
use crate::color::{colored_text, CardColor, MultiColor};
use crate::player::Player;

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
pub struct PropertyWildCard {
	pub value: u8,
	pub available_colors: MultiColor,
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

	pub fn play(mut self, color: CardColor, player: &mut Player) {
		self.set_color(color);
		player.add_property(self.into());
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
