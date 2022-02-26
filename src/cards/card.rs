use std::fmt;

use crate::cards::{ActionCardKind, MoneyCard, PropertyCard, PropertyWildCard, RentCard};
use crate::color::CardColor;
use crate::game::{read_color, Turn};
use crate::player::Assets;

pub trait Colored {
	fn set_color(&mut self, color: CardColor);
	fn colors(&self) -> Vec<CardColor>;
}

pub trait Card {
	fn value(&self) -> u8;
}

pub trait Play {
	fn can_play(&self, assets: &Assets) -> bool;
	fn play(self, turn: &mut Turn);
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum CardKind {
	PropertyCard(PropertyCard),
	ActionCard(ActionCardKind),
	MoneyCard(MoneyCard),
	RentCard(RentCard),
	PropertyWildCard(PropertyWildCard),
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum BankableCardKind {
	MoneyCard(MoneyCard),
	ActionCard(ActionCardKind),
	RentCard(RentCard),
}

impl Card for BankableCardKind {
	fn value(&self) -> u8 {
		match self {
			Self::ActionCard(c) => c.value(),
			Self::MoneyCard(c) => c.value(),
			Self::RentCard(c) => c.value(),
		}
	}
}

impl Play for CardKind {
	fn can_play(&self, assets: &Assets) -> bool {
		match self {
			Self::ActionCard(c) => c.can_play(assets),
			Self::MoneyCard(c) => c.can_play(assets),
			Self::RentCard(c) => c.can_play(assets),
			Self::PropertyCard(c) => c.can_play(assets),
			Self::PropertyWildCard(c) => c.can_play(assets),
		}
	}

	fn play(self, turn: &mut Turn) {
		match self {
			Self::ActionCard(c) => c.play(turn),
			Self::MoneyCard(c) => c.play(turn),
			Self::PropertyCard(c) => c.play(turn),
			Self::PropertyWildCard(c) => play_colored_card(c, turn),
			Self::RentCard(c) => play_colored_card(c, turn),
		}
	}
}

impl CardKind {
	pub fn play(self, turn: &mut Turn) {
		match self {
			Self::ActionCard(c) => c.play(turn),
			Self::MoneyCard(c) => c.play(turn),
			Self::PropertyCard(c) => c.play(turn),
			Self::PropertyWildCard(c) => play_colored_card(c, turn),
			Self::RentCard(c) => play_colored_card(c, turn),
		}
	}
}

fn play_colored_card<T: Play + Colored>(mut card: T, turn: &mut Turn) {
	card.set_color(read_color(&card));
	card.play(turn);
}

impl From<PropertyCard> for CardKind {
	fn from(property_card: PropertyCard) -> Self {
		Self::PropertyCard(property_card)
	}
}

impl From<ActionCardKind> for CardKind {
	fn from(action_card: ActionCardKind) -> Self {
		Self::ActionCard(action_card)
	}
}

impl From<MoneyCard> for CardKind {
	fn from(money_card: MoneyCard) -> Self {
		Self::MoneyCard(money_card)
	}
}

impl From<RentCard> for CardKind {
	fn from(rent_card: RentCard) -> Self {
		Self::RentCard(rent_card)
	}
}

impl From<PropertyWildCard> for CardKind {
	fn from(property_wild_card: PropertyWildCard) -> Self {
		Self::PropertyWildCard(property_wild_card)
	}
}

impl From<MoneyCard> for BankableCardKind {
	fn from(money_card: MoneyCard) -> Self {
		Self::MoneyCard(money_card)
	}
}

impl From<ActionCardKind> for BankableCardKind {
	fn from(action_card: ActionCardKind) -> Self {
		Self::ActionCard(action_card)
	}
}

impl From<RentCard> for BankableCardKind {
	fn from(rent_card: RentCard) -> Self {
		Self::RentCard(rent_card)
	}
}

impl fmt::Display for CardKind {
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

impl fmt::Display for BankableCardKind {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::ActionCard(c) => c.fmt(f),
			Self::MoneyCard(c) => c.fmt(f),
			Self::RentCard(c) => c.fmt(f),
		}
	}
}
