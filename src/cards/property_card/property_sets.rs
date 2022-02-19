use std::{
	collections::{hash_map::Entry, HashMap},
	fmt,
};

use crate::color::CardColor;

use crate::cards::{CardSet, PropertyCardKind};

#[derive(Debug)]
pub struct PropertySets(HashMap<CardColor, CardSet<PropertyCardKind>>);

impl PropertySets {
	pub fn new() -> Self {
		Self(HashMap::new())
	}

	pub fn entry(&mut self, color: CardColor) -> Entry<'_, CardColor, CardSet<PropertyCardKind>> {
		self.0.entry(color)
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
