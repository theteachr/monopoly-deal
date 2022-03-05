use std::fmt;

use crate::cards::{ActionCard, MoneyCard, PropertyCard, PropertyWildCard, RentCard};
use crate::game::Turn;
use crate::player::Assets;

pub trait Card {
	fn value(&self) -> u8;
	fn is_playable(&self, assets: &Assets) -> bool;
}

/// Represents all possible types a card can be.
#[derive(Debug, Hash, Eq, PartialEq)]
pub enum CardKind {
	/// Represents a property card.
	PropertyCard(PropertyCard),

	/// Represents an action card.
	ActionCard(ActionCard),

	/// Represents a money card.
	MoneyCard(MoneyCard),

	/// Represents a rent card.
	RentCard(RentCard),

	/// Represents a property wild card.
	PropertyWildCard(PropertyWildCard),
}

/// Represents cards that can be banked (played as money).
///
/// Once banked, a card cannot be used to activate its original action,
/// except for `MoneyCard` as their sole purpose is to be banked :P
#[derive(Debug, Eq, PartialEq, Hash)]
pub enum BankableCardKind {
	MoneyCard(MoneyCard),
	ActionCard(ActionCard),
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

	fn is_playable(&self, assets: &Assets) -> bool {
		match self {
			Self::ActionCard(c) => c.is_playable(assets),
			Self::MoneyCard(c) => c.is_playable(assets),
			Self::RentCard(c) => c.is_playable(assets),
		}
	}
}

impl Card for CardKind {
	fn value(&self) -> u8 {
		match self {
			Self::ActionCard(c) => c.value(),
			Self::MoneyCard(c) => c.value(),
			Self::RentCard(c) => c.value(),
			Self::PropertyCard(c) => c.value(),
			Self::PropertyWildCard(c) => c.value(),
		}
	}

	fn is_playable(&self, assets: &Assets) -> bool {
		match self {
			Self::ActionCard(c) => c.is_playable(assets),
			Self::MoneyCard(c) => c.is_playable(assets),
			Self::RentCard(c) => c.is_playable(assets),
			Self::PropertyCard(c) => c.is_playable(assets),
			Self::PropertyWildCard(c) => c.is_playable(assets),
		}
	}
}

impl CardKind {
	pub fn play(self, turn: &mut Turn) {
		match self {
			Self::ActionCard(_) => todo!(),
			Self::MoneyCard(c) => c.play(turn),
			Self::PropertyCard(c) => c.play(turn),
			Self::PropertyWildCard(c) => c.play(turn),
			Self::RentCard(mut c) => c.play(turn),
		}
	}
}

impl From<PropertyCard> for CardKind {
	fn from(property_card: PropertyCard) -> Self {
		Self::PropertyCard(property_card)
	}
}

impl From<ActionCard> for CardKind {
	fn from(action_card: ActionCard) -> Self {
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

impl From<ActionCard> for BankableCardKind {
	fn from(action_card: ActionCard) -> Self {
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
