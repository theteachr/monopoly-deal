use crate::color;
use crate::cards::rent_vec::RentVec;

use std::{
	hash::{Hash, Hasher},
	cmp::PartialEq,
};


#[derive(Debug)]
pub struct PropertyCard {
	pub title: &'static str,
	pub color: color::Color,
	pub rents: RentVec,
}


impl PropertyCard {
	pub fn new(title: &'static str, color: color::Color, rents: RentVec) -> Self {
		Self { title, color, rents }
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
