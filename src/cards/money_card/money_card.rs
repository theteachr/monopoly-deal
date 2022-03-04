use super::denomination::Denomination;
use crate::cards::Card;
use crate::game::Turn;
use crate::player::Assets;
use std::{cmp::PartialEq, fmt, hash::Hash};

/// Represents a money card.
#[derive(Debug, Hash, Eq, PartialEq)]
pub struct MoneyCard(Denomination);

impl MoneyCard {
	/// Returns a money card valued `value`.
	pub fn new(value: u8) -> Self {
		Self(value.into())
	}

	pub fn play(self, turn: &mut Turn) {
		// Simple add the card into player's assets.
		turn.assets.add_money(self.into());
	}
}

impl Card for MoneyCard {
	/// Returns the value of the card.
	fn value(&self) -> u8 {
		self.0 as u8
	}

	fn is_playable(&self, _: &Assets) -> bool {
		// `MoneyCard`s are always playable, so return `true`.
		true
	}
}

impl fmt::Display for MoneyCard {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}M", self.0 as u8)
	}
}
