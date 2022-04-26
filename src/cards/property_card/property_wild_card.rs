use std::{cmp::PartialEq, fmt, hash::Hash};

use super::{PropertyCardKind, PropertySets};
use crate::cards::Card;
use crate::color::{colored_text, CardColor, MultiColor};
use crate::common::print_read_index;
use crate::errors::NotPlayable;
use crate::game::CurrentPlayer;

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

	fn set_color(&mut self, color: CardColor) {
		self.selected_color = Some(color);
	}

	pub fn play(mut self, current_player: &mut CurrentPlayer) {
		// Get available colors as a vector as we want to be able to index (user's input) into it and set the color.
		// FIXME Doing this twice, first time in `is_playable`. Unite both methods?
		let colors = self
			.available_colors
			.get()
			.into_iter()
			.collect::<Vec<CardColor>>();

		let idx = print_read_index("> ", colors.iter(), colors.len());
		let color = colors[idx];

		self.set_color(color);
		current_player.assets.add_property(self.into());
	}

	fn is_playable_with(&self, color: CardColor, properties: &PropertySets) -> bool {
		!properties.is_complete_set(color)
	}
}

impl Card for PropertyWildCard {
	fn value(&self) -> u8 {
		self.value
	}

	fn is_playable(&self, properties: &PropertySets) -> Result<(), NotPlayable> {
		// Get available colors as a vector as we want to be able to index (user's input) into it and set the color.
		let colors = self
			.available_colors
			.get()
			.into_iter()
			.collect::<Vec<CardColor>>();

		let idx = print_read_index("> ", colors.iter(), colors.len());
		let color = colors[idx];

		if self.is_playable_with(color, properties) {
			return Ok(());
		}

		Err(NotPlayable(format!("{} is already a complete set.", color)))
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
