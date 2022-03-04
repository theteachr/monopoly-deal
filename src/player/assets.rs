use std::fmt;

use crate::{
	cards::{data::COLLECTIONS, BankableCardKind, Card, CardSet, PropertyCardKind, PropertySets},
	color::CardColor,
};

/// Holds all asset cards played by the player.
///
/// Maintains cards played as money and property cards separately.
#[derive(Debug)]
pub struct Assets {
	/// Holds cards played as money.
	pub bank: CardSet<BankableCardKind>,

	/// Holds all property cards played by the player.
	pub property_sets: PropertySets,
}

impl Assets {
	pub fn new() -> Self {
		Self {
			bank: CardSet::new(),
			property_sets: PropertySets::new(),
		}
	}

	/// Adds the card to the `bank`.
	pub fn add_money(&mut self, card: BankableCardKind) {
		self.bank.add(card);
	}

	/// Inserts the card into `property_sets`.
	pub fn add_property(&mut self, card: PropertyCardKind) {
		let color = match card {
			PropertyCardKind::Single(ref c) => c.color,
			PropertyCardKind::Wild(ref c) => c.selected_color.unwrap(),
		};

		self.property_sets
			.entry(color)
			.or_insert(CardSet::new())
			.add(card);
	}

	/// Returns the amount of rent that the player will be paid,
	/// if they choose to ask rent for all their `color` properties.
	pub fn rent(&self, color: CardColor) -> u8 {
		self.property_sets
			.cards(color)
			.map(CardSet::len)
			.map_or(0, |i| COLLECTIONS[color as usize].1[(i - 1) as usize])
	}

	/// Returns the max amount of money a player can pay using the cards in their bank.
	pub fn bank_value(&self) -> u8 {
		self.bank.iter().map(Card::value).sum()
	}

	/// Returns the max amount of money a player can pay using their properties.
	pub fn total_property_value(&self) -> u8 {
		self.property_sets.total_value()
	}
}

impl fmt::Display for Assets {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"Bank: {}\n\nProperties: {}",
			self.bank, self.property_sets
		)
	}
}
