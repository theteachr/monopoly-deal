use crate::cards::money_card::MoneyCard;
use crate::cards::property_card::PropertyCard;

use std::{
	fmt::{Debug, Formatter, Result},
	hash::{Hash, Hasher},
};

#[derive(Eq, PartialEq)]
pub enum CardType {
	Property(PropertyCard),
	Money(MoneyCard),
}

#[derive(Eq, PartialEq)]
pub struct Card {
	value: u8,
	card_type: CardType,
}

impl Card {
	pub fn new(value: u8, card_type: CardType) -> Self {
		Card { value, card_type }
	}
}

impl Debug for Card {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(
			f,
			"<{}>",
			match &self.card_type {
				CardType::Property(p) => format!(
					"Property: '{}', {}, {:?}, {}",
					p.title,
					p.color,
					p.set,
					self.value
				),
				CardType::Money(_) => format!("Money: {}", self.value),
			}
		)
	}
}

impl Hash for Card {
	fn hash<H: Hasher>(&self, state: &mut H) {
		match &self.card_type {
			CardType::Property(p) => p.hash(state),
			CardType::Money(m) => {
				m.id.hash(state);
				self.value.hash(state);
			}
		}
	}
}
