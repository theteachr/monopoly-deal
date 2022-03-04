use std::fmt;

use crate::{
	cards::{Card, Colored},
	color::{CardColor, MultiColor},
	game::{read_color, Turn},
	player::Assets,
};

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct RentCard {
	value: u8,
	available_colors: MultiColor,
	selected_color: Option<CardColor>,
}

impl RentCard {
	pub fn new(value: u8, colors: MultiColor) -> Self {
		Self {
			value,
			available_colors: colors,
			selected_color: None,
		}
	}

	pub fn play(&mut self, turn: &mut Turn) {
		// Read the color from the player.
		let color = read_color(self);

		// Assign the read color to the card.
		self.set_color(color);

		println!(
			"Playing a rent card: {}",
			turn.assets.rent(self.selected_color.unwrap())
		);
	}
}

impl Card for RentCard {
	fn value(&self) -> u8 {
		self.value
	}

	fn is_playable(&self, assets: &Assets) -> bool {
		self.colors()
			.iter()
			.any(|color| assets.property_sets.exists(color))
	}
}

// TODO Allow play only if the player owns at least one property whose color is on the `RentCard`

// FIXME Only All color wild cards need to ask for player selection
// Dual color cards should request money from the rest of the players
impl Colored for RentCard {
	fn set_color(&mut self, color: CardColor) {
		self.selected_color = Some(color);
	}

	fn colors(&self) -> Vec<CardColor> {
		Vec::from(self.available_colors)
	}
}

impl fmt::Display for RentCard {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "RentCard {}", self.available_colors)
	}
}
