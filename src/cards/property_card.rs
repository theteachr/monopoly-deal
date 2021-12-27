use std::{
	cmp::PartialEq,
	fmt,
	hash::{Hash, Hasher},
};

use crate::cards::{multi_color_card::PropertyWildCard, RentVec};
use crate::color::{colored_text, Color};

#[derive(Debug, Eq)]
pub struct PropertyCard {
	value: u8,
	name: &'static str,
	color: Color,
	rents: RentVec,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum PropertyCardKind {
	Single(PropertyCard),
	Wild(PropertyWildCard),
}

impl PropertyCard {
	pub fn new(value: u8, name: &'static str, color: Color, rents: RentVec) -> Self {
		Self {
			value,
			name,
			color,
			rents,
		}
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

impl fmt::Display for PropertyCard {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		colored_text(self.name, self.color).fmt(f)
	}
}

impl fmt::Display for PropertyCardKind {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Single(s) => s.fmt(f),
			Self::Wild(w) => w.fmt(f),
		}
	}
}
