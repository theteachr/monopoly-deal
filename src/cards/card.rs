use crate::cards::money_card::MoneyCard;
use crate::cards::property_card::PropertyCard;
use std::fmt;

pub enum CardType {
	Property(PropertyCard),
	Money(MoneyCard),
}

pub struct Card {
	value: u8,
	card_type: CardType,
}

impl Card {
	pub fn new(value: u8, card_type: CardType) -> Self {
		Card { value, card_type }
	}
}

impl fmt::Debug for Card {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"<{}>",
			match &self.card_type {
				CardType::Property(p) => format!(
					"Property: '{}', {}, {:?}, {}",
					p.title,
					p.color.to_string(),
					p.set,
					self.value
				),
				CardType::Money(_) => format!("Money: {}", self.value),
			}
		)
	}
}
