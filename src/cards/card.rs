use std::fmt;

use crate::cards::{ActionCard, MoneyCard, MultiColorCard, PropertyCard};

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Card {
	Property(PropertyCard),
	Money(MoneyCard),
	Action(ActionCard),
	Wild(MultiColorCard),
}

impl fmt::Display for Card {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use Card::*;

		match self {
			Property(c) => c.fmt(f),
			Money(c) => c.fmt(f),
			Action(c) => c.fmt(f),
			Wild(c) => c.fmt(f),
		}
	}
}
