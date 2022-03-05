use std::fmt;

use crate::cards::{ActionCard, MoneyCard, PropertyCard, PropertyWildCard, RentCard};
use crate::game::CurrentPlayer;
use crate::player::Assets;

pub trait Card {
	fn value(&self) -> u8;
	fn is_playable(&self, assets: &Assets) -> bool;
}

macro_rules! apply_inner {
	( $VAL:ident $ENUM:ident $CARD:ident { $EXPR:expr } $($VARIANTS:ident)+ ) => {
		match $VAL { $( $ENUM::$VARIANTS($CARD)=> $EXPR, )+ }
	};
}

macro_rules! card_kind_apply_inner {
	( $VAL:ident $CARD:ident => $EXPR:expr ) => {
		apply_inner! { $VAL CardKind $CARD {$EXPR} ActionCard MoneyCard RentCard PropertyCard PropertyWildCard }
	};
}

macro_rules! bankable_card_kind_apply_inner {
	( $VAL:ident $CARD:ident => $EXPR:expr ) => {
		apply_inner! { $VAL BankableCardKind $CARD {$EXPR} ActionCard MoneyCard RentCard }
	};
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
		bankable_card_kind_apply_inner!(self c => c.value())
	}

	fn is_playable(&self, assets: &Assets) -> bool {
		bankable_card_kind_apply_inner!(self c => c.is_playable(assets))
	}
}

impl Card for CardKind {
	fn value(&self) -> u8 {
		card_kind_apply_inner!(self c => c.value())
	}

	fn is_playable(&self, assets: &Assets) -> bool {
		card_kind_apply_inner!(self c => c.is_playable(assets))
	}
}

impl CardKind {
	pub fn play(self, current_player: &mut CurrentPlayer) {
		card_kind_apply_inner!(self c => c.play(current_player))
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
		card_kind_apply_inner!(self c => c.fmt(f))
	}
}

impl fmt::Display for BankableCardKind {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		bankable_card_kind_apply_inner!(self c => c.fmt(f))
	}
}
