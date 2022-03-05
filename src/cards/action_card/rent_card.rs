use std::fmt;

use crate::{
	cards::Card,
	color::{CardColor, MultiColor},
	common::read_index,
	game::Turn,
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

	// should be called only if playable
	// if that's guaranteed, we know that there will be at least one color
	pub fn play(&mut self, turn: &mut Turn) {
		// get the colors the player can play given their assets
		let playable_colors = self
			.available_colors
			.colors()
			.intersection(&turn.assets.property_sets.colors())
			.cloned()
			.collect::<Vec<CardColor>>();

		// if there are more than one colors, let the player choose, else we choose the one at index 0, as it's the only one
		let index = if playable_colors.len() > 1 {
			read_index("> ", playable_colors.iter(), playable_colors.len())
		} else {
			0
		};

		// read the index of the color from the player, and get the color at it
		let color = playable_colors[index];

		// assign the read color to the card
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
