use std::collections::HashSet;

use crate::cards::{
	money_card::MoneyCard,
	property_card::PropertyCard,
};

#[derive(Debug)]
pub struct PlayerState<'a> {
	name: String,
	properties: HashSet<&'a PropertyCard>,
	bank: HashSet<&'a MoneyCard>,
}

impl PlayerState<'_> {
	pub fn new(name: String) -> Self {
		Self { name, properties: HashSet::new(), bank: HashSet::new() }
	}
}
