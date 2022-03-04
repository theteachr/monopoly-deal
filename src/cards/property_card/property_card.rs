use std::{
	cmp::PartialEq,
	fmt,
	hash::{Hash, Hasher},
};

use super::PropertyCardKind;
use crate::cards::{data::COLLECTIONS, Card};
use crate::color::{colored_text, CardColor};
use crate::game::Turn;
use crate::player::Assets;

/// Represents a mono colored property card.
#[derive(Debug, Eq)]
pub struct PropertyCard {
	pub name: &'static str,
	pub color: CardColor,
}

impl PropertyCard {
	pub fn new(name: &'static str, color: CardColor) -> Self {
		Self { name, color }
	}

	pub fn play(self, turn: &mut Turn) {
		turn.assets.add_property(self.into());
	}
}

impl Card for PropertyCard {
	fn value(&self) -> u8 {
		COLLECTIONS[self.color as usize].0
	}

	fn is_playable(&self, _: &Assets) -> bool {
		true
	}
}

impl Hash for PropertyCard {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.name.hash(state);
	}
}

impl PartialEq for PropertyCard {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name
	}
}

impl std::convert::From<PropertyCard> for PropertyCardKind {
	fn from(property_card: PropertyCard) -> Self {
		Self::Single(property_card)
	}
}

impl fmt::Display for PropertyCard {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", colored_text(self.name, self.color),)
	}
}

impl fmt::Display for PropertyCardKind {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Single(c) => c.fmt(f),
			Self::Wild(c) => c.fmt(f),
		}
	}
}
