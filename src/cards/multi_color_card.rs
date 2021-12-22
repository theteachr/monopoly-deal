use std::{cmp::PartialEq, fmt, hash::Hash};

use crate::color::{colored_text, Color, COLORS};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum MultiColor {
	Two(Color, Color),
	All,
}

#[derive(Debug, PartialEq, Hash, Eq)]
pub enum MultiColorCardType {
	Rent,
	Property,
}

#[derive(Debug, PartialEq, Hash, Eq)]
pub struct MultiColorCard {
	value: u8,
	colors: MultiColor,
	card_type: MultiColorCardType,
}

impl MultiColorCard {
	pub fn new(value: u8, colors: MultiColor, card_type: MultiColorCardType) -> Self {
		Self {
			value,
			colors,
			card_type,
		}
	}
}

impl fmt::Display for MultiColorCard {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use MultiColorCardType::*;

		let block = "██";
		let card_string = match self.card_type {
			Rent => "Rent",
			Property => "Property Wild",
		};

		let colors = match self.colors {
			MultiColor::Two(c, d) => vec![c, d],
			MultiColor::All => COLORS.to_vec(),
		};

		let colored_blocks = colors
			.iter()
			.map(|&color| colored_text(block, color))
			.collect::<Vec<String>>()
			.join("");

		write!(f, "{}Card {}", card_string, colored_blocks)
	}
}
