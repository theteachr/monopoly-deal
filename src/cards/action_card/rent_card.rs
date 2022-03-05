use std::fmt;

use crate::{
	cards::Card,
	color::{CardColor, MultiColor},
	common::{print_indexed, read_index},
	game::CurrentPlayer,
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

	fn set_color(&mut self, color: CardColor) {
		self.selected_color = Some(color);
	}

	pub fn play(&mut self, current_player: &mut CurrentPlayer) {
		// Get the colors the player can play given their assets.
		let playable_colors = self
			.available_colors
			.colors()
			.intersection(&current_player.assets.property_sets.colors())
			.cloned()
			.collect::<Vec<CardColor>>();

		// Print the colors with their index so the user can choose.
		print_indexed(playable_colors.iter());

		// This method should be called only if playable. If that's guaranteed,
		// we know that there will be at least one color. If there are more than one colors,
		// let the player choose, else we choose the one at index 0, as it's the only one.
		let index = if playable_colors.len() > 1 {
			read_index("> ", playable_colors.len())
		} else {
			0
		};

		let color = playable_colors[index];

		// Assign the read color to the card.
		self.set_color(color);

		println!(
			"Playing a rent card: {}",
			current_player.assets.rent(self.selected_color.unwrap())
		);
	}
}

impl Card for RentCard {
	fn value(&self) -> u8 {
		self.value
	}

	fn is_playable(&self, assets: &Assets) -> bool {
		self.available_colors
			.colors()
			.iter()
			.any(|color| assets.property_sets.exists(color))
	}
}

impl fmt::Display for RentCard {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "RentCard {}", self.available_colors)
	}
}
