use std::{
	hash::{Hash, Hasher},
};

#[derive(Debug)]
pub struct MoneyCard {
	pub id: String,
}

impl MoneyCard {
	pub fn new(id: String) -> Self {
		Self { id }
	}
}

impl Hash for MoneyCard {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.id.hash(state);
	}
}

impl PartialEq for MoneyCard {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl Eq for MoneyCard {}
