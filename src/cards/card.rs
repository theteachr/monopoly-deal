use std::fmt;

use crate::cards::{
	multi_color_card::{PropertyWildCard, RentCard},
	Action, ActionCard, ActionCardKind, MoneyCard, PropertyCard, PropertyCardKind, RentVec,
};
use crate::color::{Color, MultiColor};
use crate::game::{player::Player, Playable};

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Card {
	Bankable(BankableCardKind),
	Property(PropertyCardKind),
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum BankableCardKind {
	Money(MoneyCard),
	Action(ActionCardKind),
}

impl Card {
	pub fn new_property_card(value: u8, name: &'static str, color: Color, rents: RentVec) -> Self {
		Self::Property(PropertyCardKind::Single(PropertyCard::new(
			value, name, color, rents,
		)))
	}

	pub fn new_money_card(value: u8) -> Self {
		Self::Bankable(BankableCardKind::Money(MoneyCard::new(value)))
	}

	pub fn new_action_card(value: u8, action: Action) -> Self {
		Self::Bankable(BankableCardKind::Action(ActionCardKind::Action(
			ActionCard::new(value, action),
		)))
	}

	pub fn new_rent_card(value: u8, colors: MultiColor) -> Self {
		Card::Bankable(BankableCardKind::Action(ActionCardKind::Rent(
			RentCard::new(value, colors),
		)))
	}

	pub fn new_property_wild_card(value: u8, colors: MultiColor) -> Self {
		Self::Property(PropertyCardKind::Wild(PropertyWildCard::new(value, colors)))
	}
}

impl Playable for Card {
	fn play(self, player: &mut Player) {
		match self {
			Self::Bankable(b) => match b {
				BankableCardKind::Money(_) => player.played.add_money(b),
				BankableCardKind::Action(a) => a.play(player),
			},
			Self::Property(p) => p.play(player),
		}
	}
}

impl fmt::Display for BankableCardKind {
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
