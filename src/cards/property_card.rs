use crate::cards::rent_vec::RentVec;
use crate::color;

use std::{
	cmp::PartialEq,
	hash::{Hash, Hasher},
};

#[derive(Debug)]
pub struct PropertyCard {
	value: u8,
	pub title: &'static str,
	pub color: color::Color,
	pub rents: RentVec,
}

impl PropertyCard {
	pub fn new(value: u8, title: &'static str, color: color::Color, rents: RentVec) -> Self {
		Self {
			value,
			title,
			color,
			rents,
		}
	}
}

impl Hash for PropertyCard {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.title.hash(state);
	}
}

impl PartialEq for PropertyCard {
	fn eq(&self, other: &Self) -> bool {
		self.title == other.title
	}
}

impl Eq for PropertyCard {}
