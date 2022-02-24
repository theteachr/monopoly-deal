use std::fmt;

use crate::{
	cards::{Card, Colored, Play},
	color::{CardColor, MultiColor},
	player::Player,
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
}

impl Card for RentCard {
	fn value(&self) -> u8 {
		self.value
	}
}

impl Play for RentCard {
	fn can_play(&self, player: &Player) -> bool {
		self.colors()
			.iter()
			.any(|color| player.owns_property_of_color(color))
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

	fn play(self, color: CardColor, player: &mut Player) {
		println!("Playing a rent card: {}", player.rent(color));
	}
}

impl fmt::Display for RentCard {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "RentCard {}", self.available_colors)
	}
}
