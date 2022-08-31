use std::fmt;

use crate::cards::{ActionCard, MoneyCard, PropertyCard, PropertyWildCard, RentCard};
use crate::errors::NotPlayable;
use crate::entities::CurrentPlayer;
use crate::Game;

use super::PropertyCardKind;
use super::PropertySets;

pub trait Card {
	fn value(&self) -> u8;
	fn is_playable(&self, properties: &PropertySets) -> Result<(), NotPlayable>;
}

macro_rules! apply_inner {
	( $VAL:ident $ENUM:ident $CARD:ident { $EXPR:expr } $($VARIANTS:ident)+ ) => {
		match $VAL { $( $ENUM::$VARIANTS($CARD)=> $EXPR, )+ }
	};
}

macro_rules! card_kind_apply_inner {
	( $VAL:ident $CARD:ident => $EXPR:expr ) => {
		apply_inner! { $VAL CardKind $CARD {$EXPR} Action Money Rent Property PropertyWild }
	};
}

macro_rules! bankable_card_kind_apply_inner {
	( $VAL:ident $CARD:ident => $EXPR:expr ) => {
		apply_inner! { $VAL BankableCardKind $CARD {$EXPR} Action Money Rent }
	};
}

macro_rules! paid_card_kind_apply_inner {
	( $VAL:ident $CARD:ident => $EXPR:expr ) => {
		apply_inner! { $VAL PaidCardKind $CARD {$EXPR} Banked Property }
	};
}

/// Represents all possible types a card can be.
#[derive(Debug, Hash, Eq, PartialEq)]
pub enum CardKind {
	/// Represents a property card.
	Property(PropertyCard),

	/// Represents an action card.
	Action(ActionCard),

	/// Represents a money card.
	Money(MoneyCard),

	/// Represents a rent card.
	Rent(RentCard),

	/// Represents a property wild card.
	PropertyWild(PropertyWildCard),
}

/// Represents cards that can be banked (played as money).
///
/// Once banked, a card cannot be used to activate its original action,
/// except for `MoneyCard` as their sole purpose is to be banked :P
#[derive(Debug, Eq, PartialEq, Hash)]
pub enum BankableCardKind {
	Money(MoneyCard),
	Action(ActionCard),
	Rent(RentCard),
}

pub enum PaidCardKind {
	Banked(BankableCardKind),
	Property(PropertyCardKind),
}

impl Card for BankableCardKind {
	fn value(&self) -> u8 {
		bankable_card_kind_apply_inner!(self c => c.value())
	}

	fn is_playable(&self, properties: &PropertySets) -> Result<(), NotPlayable> {
		bankable_card_kind_apply_inner!(self c => c.is_playable(properties))
	}
}

impl Card for PaidCardKind {
	fn value(&self) -> u8 {
		paid_card_kind_apply_inner!(self c => c.value())
	}

	fn is_playable(&self, properties: &PropertySets) -> Result<(), NotPlayable> {
		paid_card_kind_apply_inner!(self c => c.is_playable(properties))
	}
}

impl Card for CardKind {
	fn value(&self) -> u8 {
		card_kind_apply_inner!(self c => c.value())
	}

	fn is_playable(&self, properties: &PropertySets) -> Result<(), NotPlayable> {
		card_kind_apply_inner!(self c => c.is_playable(properties))
	}
}

impl CardKind {
	pub fn play(self, current_player: &mut CurrentPlayer, game: &mut Game) {
		match self {
			Self::Action(card) => card.play(current_player, game),
			Self::Money(card) => card.play(current_player),
			Self::Property(card) => card.play(current_player),
			Self::PropertyWild(card) => card.play(current_player),
			Self::Rent(card) => card.play(&current_player.assets.property_sets),
		}
	}
}

impl From<BankableCardKind> for PaidCardKind {
	fn from(bankable_card_kind: BankableCardKind) -> Self {
		bankable_card_kind_apply_inner!(bankable_card_kind c => Self::Banked(c.into()))
	}
}

impl From<PropertyCardKind> for PaidCardKind {
	fn from(property_card_kind: PropertyCardKind) -> Self {
		match property_card_kind {
			PropertyCardKind::Single(c) => Self::Property(c.into()),
			PropertyCardKind::Wild(c) => Self::Property(c.into()),
		}
	}
}

impl From<PropertyCardKind> for CardKind {
	fn from(property_card_kind: PropertyCardKind) -> Self {
		match property_card_kind {
			PropertyCardKind::Single(c) => Self::Property(c),
			PropertyCardKind::Wild(c) => Self::PropertyWild(c),
		}
	}
}

impl From<PropertyCard> for CardKind {
	fn from(property_card: PropertyCard) -> Self {
		Self::Property(property_card)
	}
}

impl From<ActionCard> for CardKind {
	fn from(action_card: ActionCard) -> Self {
		Self::Action(action_card)
	}
}

impl From<MoneyCard> for CardKind {
	fn from(money_card: MoneyCard) -> Self {
		Self::Money(money_card)
	}
}

impl From<RentCard> for CardKind {
	fn from(rent_card: RentCard) -> Self {
		Self::Rent(rent_card)
	}
}

impl From<PropertyWildCard> for CardKind {
	fn from(property_wild_card: PropertyWildCard) -> Self {
		Self::PropertyWild(property_wild_card)
	}
}

impl From<MoneyCard> for BankableCardKind {
	fn from(money_card: MoneyCard) -> Self {
		Self::Money(money_card)
	}
}

impl From<ActionCard> for BankableCardKind {
	fn from(action_card: ActionCard) -> Self {
		Self::Action(action_card)
	}
}

impl From<RentCard> for BankableCardKind {
	fn from(rent_card: RentCard) -> Self {
		Self::Rent(rent_card)
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

impl fmt::Display for PaidCardKind {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		paid_card_kind_apply_inner!(self c => c.fmt(f))
	}
}
