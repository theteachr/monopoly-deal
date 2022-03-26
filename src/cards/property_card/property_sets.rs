use std::{
	collections::{hash_map::Entry, HashMap, HashSet},
	fmt,
};

use crate::color::CardColor;

use crate::cards::data::COLLECTIONS;
use crate::cards::{CardSet, PropertyCardKind};

/// Tracks the set of property cards played by a player.
///
/// Key represents a `CardColor`, and the associated value is a
/// set of properties of that color played by a player.
#[derive(Debug)]
pub struct PropertySets(HashMap<CardColor, CardSet<PropertyCardKind>>);

impl PropertySets {
	/// Returns an empty set of properties.
	pub fn new() -> Self {
		Self(HashMap::new())
	}

	pub fn entry(&mut self, color: CardColor) -> Entry<'_, CardColor, CardSet<PropertyCardKind>> {
		self.0.entry(color)
	}

	/// Returns all colors in the properties.
	pub fn colors(&self) -> HashSet<CardColor> {
		self.0.keys().cloned().collect()
	}

	/// Pops a card from the `color` set.
	pub fn pop(&mut self, color: &CardColor) -> PropertyCardKind {
		self.0.get_mut(color).unwrap().remove(0)
	}

	/// Returns the amount of rent that the player will be paid,
	/// if they choose to ask rent for all their `color` properties.
	pub fn rent(&self, color: CardColor) -> u8 {
		self.0.get(&color).map_or(0, |cards| {
			COLLECTIONS[color as usize].1[(cards.len() - 1) as usize]
		})
	}

	/// Returns the total value all the played properties.
	pub fn total_value(&self) -> u8 {
		self.0.values().map(CardSet::value).sum()
	}

	/// Return `true` if at least one property of the given `color` exists in the set.
	pub fn exists(&self, color: &CardColor) -> bool {
		self.0.contains_key(&color)
	}

	pub fn iter(&self) -> impl Iterator<Item = CardColor> + '_ {
		self.0.keys().cloned()
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
