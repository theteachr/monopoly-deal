use std::{
	collections::{hash_map::Entry, HashMap},
	fmt,
};

use crate::color::CardColor;

use crate::cards::{Card, CardSet, PropertyCardKind};

#[derive(Debug)]
pub struct PropertySets(HashMap<CardColor, CardSet<PropertyCardKind>>);

impl PropertySets {
	pub fn new() -> Self {
		Self(HashMap::new())
	}

	pub fn entry(&mut self, color: CardColor) -> Entry<'_, CardColor, CardSet<PropertyCardKind>> {
		self.0.entry(color)
	}

	pub fn cards(&self, color: CardColor) -> Option<&CardSet<PropertyCardKind>> {
		self.0.get(&color)
	}

	fn set_value(cards: &CardSet<PropertyCardKind>) -> u8 {
		cards.iter().map(Card::value).sum()
	}

	pub fn total_value(&self) -> u8 {
		self.0.values().map(PropertySets::set_value).sum()
	}

	pub fn exists(&self, color: CardColor) -> bool {
		self.0.contains_key(&color)
	}
}

impl fmt::Display for PropertySets {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.0.is_empty() {
			return write!(f, "[]");
		}

		let text = self
			.0
			.iter()
			.map(|(color, cards)| format!("{}: {}", color, cards))
			.collect::<Vec<String>>()
			.join("\n  ");

		write!(f, "[\n  {}\n]", text)
	}
}
