use std::{
	cmp::PartialEq,
	fmt,
	hash::{Hash, Hasher},
};

use super::PropertyCardKind;
use crate::cards::{Card, data::COLLECTIONS};
use crate::color::{colored_text, CardColor};
use crate::player::Player;

#[derive(Debug, Eq)]
pub struct PropertyCard {
	pub name: &'static str,
	pub color: CardColor,
}

impl PropertyCard {
	pub fn new(name: &'static str, color: CardColor) -> Self {
		Self { name, color }
	}

	pub fn play(self, player: &mut Player) {
		player.add_property(self.into());
	}

	pub fn rent(&self, num_cards: u8) -> u8 {
		COLLECTIONS[self.color as usize].1[num_cards as usize]
	}
}

impl Card for PropertyCard {
	fn value(&self) -> u8 {
		COLLECTIONS[self.color as usize].0
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
