use std::collections::VecDeque;
use std::fmt;

use crate::cards::{
	ActionCard, MoneyCard, PropertyCard, PropertyCardKind, PropertyWildCard, RentCard,
};
use crate::color::CardColor;
use crate::game::read_color;
use crate::player::Player;

pub trait Colored {
	fn set_color(&mut self, color: CardColor);
	fn colors(&self) -> Vec<CardColor>;
	fn play(self, color: CardColor, player: &mut Player);
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Card {
	PropertyCard(PropertyCard),
	ActionCard(ActionCard),
	MoneyCard(MoneyCard),
	RentCard(RentCard),
	PropertyWildCard(PropertyWildCard),
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum BankableCard {
	MoneyCard(MoneyCard),
	ActionCard(ActionCard),
	RentCard(RentCard),
}

impl Card {
	pub fn play(self, table: &mut VecDeque<Player>, player: &mut Player) {
		match self {
			Self::ActionCard(c) => c.play(table, player),
			Self::MoneyCard(c) => c.play(player),
			Self::PropertyCard(c) => c.play(player),
			Self::PropertyWildCard(c) => play_colored_card(c, player),
			Self::RentCard(c) => play_colored_card(c, player),
		}
	}
}

fn play_colored_card<T: Colored>(card: T, player: &mut Player) {
	let color = read_color(&card);
	card.play(color, player);
}

impl From<PropertyCard> for Card {
	fn from(property_card: PropertyCard) -> Self {
		Self::PropertyCard(property_card)
	}
}

impl From<ActionCard> for Card {
	fn from(action_card: ActionCard) -> Self {
		Self::ActionCard(action_card)
	}
}

impl From<MoneyCard> for Card {
	fn from(money_card: MoneyCard) -> Self {
		Self::MoneyCard(money_card)
	}
}

impl From<RentCard> for Card {
	fn from(rent_card: RentCard) -> Self {
		Self::RentCard(rent_card)
	}
}

impl From<PropertyWildCard> for Card {
	fn from(property_wild_card: PropertyWildCard) -> Self {
		Self::PropertyWildCard(property_wild_card)
	}
}

impl fmt::Display for Card {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::ActionCard(c) => c.fmt(f),
			Self::MoneyCard(c) => c.fmt(f),
			Self::PropertyCard(c) => c.fmt(f),
			Self::PropertyWildCard(c) => c.fmt(f),
			Self::RentCard(c) => c.fmt(f),
		}
	}
}

impl From<MoneyCard> for BankableCard {
	fn from(money_card: MoneyCard) -> Self {
		Self::MoneyCard(money_card)
	}
}

impl From<ActionCard> for BankableCard {
	fn from(action_card: ActionCard) -> Self {
		Self::ActionCard(action_card)
	}
}

impl From<RentCard> for BankableCard {
	fn from(rent_card: RentCard) -> Self {
		Self::RentCard(rent_card)
	}
}

impl fmt::Display for BankableCard {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::ActionCard(c) => c.fmt(f),
			Self::MoneyCard(c) => c.fmt(f),
			Self::RentCard(c) => c.fmt(f),
		}
	}
}

impl fmt::Display for PropertyCardKind {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Single(c) => c.fmt(f),
			Self::Wild(c) => c.fmt(f),
		}
	}
}
