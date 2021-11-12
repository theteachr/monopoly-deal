use std::collections::HashSet;

use crate::cards::{
    money_card::MoneyCard,
    property_card::PropertyCard,
};

#[derive(Debug)]
pub struct PlayerState<'a> {
	properties: HashSet<&'a PropertyCard>,
	bank: HashSet<&'a MoneyCard>,
}

impl PlayerState<'_> {
	pub fn new() -> Self {
		Self { properties: HashSet::new(), bank: HashSet::new() }
	}
}
