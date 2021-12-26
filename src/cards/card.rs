use std::fmt;

use crate::cards::{
	Action, ActionCard, EActionCard, EPropertyCard, MoneyCard, PropertyCard, PropertyWildCard,
	RentCard, RentVec,
};
use crate::color::{Color, MultiColor};

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Card {
	Bankable(EBankableCard),
	Property(EPropertyCard),
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum EBankableCard {
	Money(MoneyCard),
	Action(EActionCard),
}

impl Card {
	pub fn new_property_card(value: u8, name: &'static str, color: Color, rents: RentVec) -> Self {
		Self::Property(EPropertyCard::Single(PropertyCard::new(
			value, name, color, rents,
		)))
	}

	pub fn new_money_card(value: u8) -> Self {
		Self::Bankable(EBankableCard::Money(MoneyCard::new(value)))
	}

	pub fn new_action_card(value: u8, action: Action) -> Self {
		Self::Bankable(EBankableCard::Action(EActionCard::Action(ActionCard::new(
			value, action,
		))))
	}

	pub fn new_rent_card(value: u8, colors: MultiColor) -> Self {
		Card::Bankable(EBankableCard::Action(EActionCard::Rent(RentCard::new(
			value, colors,
		))))
	}

	pub fn new_property_wild_card(value: u8, colors: MultiColor) -> Self {
		Self::Property(EPropertyCard::Wild(PropertyWildCard::new(value, colors)))
	}
}

impl fmt::Display for EBankableCard {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Money(m) => m.fmt(f),
			Self::Action(a) => a.fmt(f),
		}
	}
}

impl fmt::Display for Card {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Bankable(b) => b.fmt(f),
			Self::Property(p) => p.fmt(f),
		}
	}
}
