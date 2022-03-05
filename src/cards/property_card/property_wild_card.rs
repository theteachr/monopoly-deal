use std::{cmp::PartialEq, fmt, hash::Hash};

use super::PropertyCardKind;
use crate::cards::Card;
use crate::color::{colored_text, CardColor, MultiColor};
use crate::common::{print_indexed, read_index};
use crate::game::CurrentPlayer;
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

	fn set_color(&mut self, color: CardColor) {
		self.selected_color = Some(color);
	}

	// FIXME Rent Card has a similar logic, the code isn't dry.
	pub fn play(mut self, current_player: &mut CurrentPlayer) {
		// Get available colors as a vector as we want to be able to index (user's input) into it and set the color.
		let colors = self
			.available_colors
			.colors()
			.into_iter()
			.collect::<Vec<CardColor>>();

		// Print the indexed list of colors for the user to pick.
		print_indexed(colors.iter());

		// read color from the player
		let color = colors[read_index("> ", colors.len())];

		// set the read color to the card
		self.set_color(color);

		// add the card into player's properties
		current_player.assets.add_property(self.into());
	}
}

impl Card for PropertyWildCard {
	fn value(&self) -> u8 {
		self.value
	}

	fn is_playable(&self, _: &Assets) -> bool {
		true
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
