use std::fmt;

use crate::{
	cards::{Card, PropertySets},
	color::{CardColor, MultiColor},
	common::print_read_index,
	errors::NotPlayable,
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

	pub fn play(mut self, properties: &PropertySets) {
		// Get the colors the player can play given their assets.
		let playable_colors = self
			.available_colors
			.get()
			.intersection(&properties.colors())
			.cloned()
			.collect::<Vec<CardColor>>();

		// This method should be called only if playable. If that's guaranteed,
		// we know that there will be at least one color. If there are more than one colors,
		// let the player choose, else we choose the one at index 0, as it's the only one.
		let index = if playable_colors.len() > 1 {
			print_read_index("> ", playable_colors.iter(), playable_colors.len())
		} else {
			0
		};

		let color = playable_colors[index];

		// Assign the read color to the card.
		self.set_color(color);

		println!(
			"Playing a rent card: {}",
			properties.rent(self.selected_color.unwrap())
		);

		// TODO Ask all players for rent. If it's an all colored rent card, target a single player
		// for grabbing the rent.
	}
}

impl Card for RentCard {
	fn value(&self) -> u8 {
		self.value
	}

	fn is_playable(&self, properties: &PropertySets) -> Result<(), NotPlayable> {
		// Check if any of the colors on the card is present in the properties played by the player.
		// If yes, then report as playable.
		if self
			.available_colors
			.get()
			.iter()
			.any(|color| properties.exists(color))
		{
			return Ok(());
		}

		// The player didn't own any property of any color on the card,
		// report with an appropriate error message.
		Err(NotPlayable(format!(
			"You need to own at least one property colored in any of {}.",
			self.available_colors
		)))
	}
}

impl fmt::Display for RentCard {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "RentCard {}", self.available_colors)
	}
}
