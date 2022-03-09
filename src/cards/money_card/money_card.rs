use super::denomination::Denomination;
use crate::cards::PropertySets;
use crate::game::CurrentPlayer;
use crate::{cards::Card, errors::NotPlayable};
use std::{cmp::PartialEq, fmt, hash::Hash};

/// Represents a money card.
#[derive(Debug, Hash, Eq, PartialEq)]
pub struct MoneyCard(Denomination);

impl MoneyCard {
	/// Returns a money card valued `value`.
	pub fn new(value: u8) -> Self {
		Self(value.into())
	}

	pub fn play(self, current_player: &mut CurrentPlayer) {
		// Simple add the card into player's assets.
		current_player.assets.add_money(self.into());
	}
}

impl Card for MoneyCard {
	/// Returns the value of the card.
	fn value(&self) -> u8 {
		self.0 as u8
	}

	fn is_playable(&self, _: &PropertySets) -> Result<(), NotPlayable> {
		// `MoneyCard`s are always playable, so return `true`.
		Ok(())
	}
}

impl fmt::Display for MoneyCard {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}M", self.0 as u8)
	}
}
