use std::{
	collections::{hash_map::Entry, HashMap, HashSet},
	fmt,
};

use crate::{cards::PaidCardKind, color::CardColor};

use crate::cards::data::COLLECTIONS;
use crate::cards::{CardSet, PropertyCardKind};

/// Tracks the set of property cards played by a player.
///
#[derive(Debug)]
pub struct PropertySets {
	/// A map from color to set of cards, where the value holds all cards of
	/// that color.
	properties: HashMap<CardColor, CardSet<PropertyCardKind>>,

	/// A list of colors that are complete sets.
	sets: Vec<CardColor>,
}

// TODO Add logic to track color sets.

impl PropertySets {
	/// Returns an empty set of properties.
	pub fn new() -> Self {
		Self {
			properties: HashMap::new(),
			sets: Vec::new(),
		}
	}

	pub fn entry(&mut self, color: CardColor) -> Entry<'_, CardColor, CardSet<PropertyCardKind>> {
		self.properties.entry(color)
	}

	/// Returns all colors in the properties.
	pub fn colors(&self) -> HashSet<CardColor> {
		self.properties.keys().cloned().collect()
	}

	/// Pops a card from the `color` set.
	pub fn pop(&mut self, color: &CardColor) -> PropertyCardKind {
		let cards = self.properties.get_mut(color).unwrap();
		let popped = cards.remove(0);

		if cards.is_empty() {
			self.properties.remove(color);
		}

		popped
	}

	/// Returns the amount of rent that the player will be paid,
	/// if they choose to ask rent for all their `color` properties.
	pub fn rent(&self, color: CardColor) -> u8 {
		self.properties
			.get(&color)
			.map_or(0, |cards| rent_for(color, cards.len()))
	}

	/// Returns the total value all the played properties.
	pub fn total_value(&self) -> u8 {
		self.properties.values().map(CardSet::value).sum()
	}

	/// Returns `true` if at least one property of the given `color` exists in the set.
	pub fn exists(&self, color: &CardColor) -> bool {
		self.properties.contains_key(&color)
	}

	/// Returns an iterator over the colors played by the players.
	pub fn iter(&self) -> impl Iterator<Item = CardColor> + '_ {
		self.properties.keys().cloned()
	}

	/// Returns all properties and empties the player property assets.
	pub fn go_popper(&mut self) -> Vec<PaidCardKind> {
		let cards: Vec<PaidCardKind> = self
			.properties
			.values_mut()
			.map(|property_cards| property_cards.remove_all())
			.flatten()
			.map(PaidCardKind::from)
			.collect();

		self.properties.clear();

		cards
	}
}

impl fmt::Display for PropertySets {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.properties.is_empty() {
			return write!(f, "[]");
		}

		let text = self
			.properties
			.iter()
			.map(|(color, cards)| format!("{}: {}", color, cards))
			.collect::<Vec<String>>()
			.join("\n  ");

		write!(f, "[\n  {}\n]", text)
	}
}

/// Returns the array of rents for the `color`.
fn rents(color: CardColor) -> &'static [u8] {
	COLLECTIONS[color as usize].1
}

/// Returns the amount of rent the player will be paid for `num_cards` of `color`.
fn rent_for(color: CardColor, num_cards: usize) -> u8 {
	rents(color)[(num_cards - 1) as usize]
}

/// Returns the number of cards required for a complete set for the `color`.
fn num_cards_for_complete_set(color: CardColor) -> usize {
	rents(color).len() as usize
}
