use std::fmt;

use crate::{
	cards::{data::COLLECTIONS, BankableCardKind, Card, CardSet, PropertyCardKind, PropertySets},
	color::CardColor,
};

#[derive(Debug)]
pub struct Assets {
	pub bank: CardSet<BankableCardKind>,
	pub property_sets: PropertySets,
}

impl Assets {
	pub fn new() -> Self {
		Self {
			bank: CardSet::new(),
			property_sets: PropertySets::new(),
		}
	}

	pub fn add_money(&mut self, card: BankableCardKind) {
		self.bank.add(card);
	}

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

	pub fn rent(&self, color: CardColor) -> u8 {
		self.property_sets
			.cards(color)
			.map(CardSet::len)
			.map(|i| COLLECTIONS[color as usize].1[(i - 1) as usize])
			.unwrap_or(0)
	}

	pub fn bank_value(&self) -> u8 {
		self.bank.iter().map(Card::value).sum()
	}

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
