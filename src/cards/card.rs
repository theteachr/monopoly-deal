use std::fmt;

use crate::cards::money_card::MoneyCard;
use crate::cards::property_card::PropertyCard;

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Card {
	Property(PropertyCard),
	Money(MoneyCard),
}

impl fmt::Display for Card {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let string = match self {
			Card::Property(p) => p.to_string(),
			Card::Money(m) => m.to_string(),
		};

		write!(f, "{}", string)
	}
}
