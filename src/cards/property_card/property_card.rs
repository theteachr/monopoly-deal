use std::{
	cmp::PartialEq,
	fmt,
	hash::{Hash, Hasher},
};

use super::{PropertyCardKind, RentVec};
use crate::color::{colored_text, CardColor};
use crate::player::Player;

#[derive(Debug, Eq)]
pub struct PropertyCard {
	pub value: u8,
	pub name: &'static str,
	pub color: CardColor,
	pub rents: RentVec,
}

impl PropertyCard {
	pub fn new(value: u8, name: &'static str, color: CardColor, rents: RentVec) -> Self {
		Self {
			value,
			name,
			color,
			rents,
		}
	}

	pub fn play(self, player: &mut Player) {
		player.add_property(self.into());
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
		write!(
			f,
			"{} ({}) {:?}",
			colored_text(self.name, self.color),
			self.value,
			self.rents
		)
	}
}
