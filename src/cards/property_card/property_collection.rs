use std::collections::HashMap;

use crate::color::CardColor;

#[derive(Debug)]
pub struct  PropertyCardInfo {
	value: u8,
	rents: &'static [u8],
}

impl PropertyCardInfo {
	pub fn new(value: u8, rents: &'static [u8]) -> Self {
		Self {
			value,
			rents,
		}
	}
}

#[derive(Debug)]
pub struct PropertyCollection(HashMap<CardColor, PropertyCardInfo>);

impl PropertyCollection {
	pub fn from_array_of_triples(triples: [(CardColor, u8, &'static [u8]); 10]) -> Self {
		let mut property_collection = HashMap::new();

		for (color, value, rents) in triples {
			property_collection.insert(color, PropertyCardInfo::new(value, rents));
		}

		Self(property_collection)
	}
}