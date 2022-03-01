use super::denomination::Denomination;
use crate::cards::{Card, Play};
use crate::game::Turn;
use crate::player::Assets;
use std::{cmp::PartialEq, fmt, hash::Hash};

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct MoneyCard(Denomination);

impl MoneyCard {
	pub fn new(value: u8) -> Self {
		Self(value.into())
	}
}

impl Card for MoneyCard {
	fn value(&self) -> u8 {
		self.0 as u8
	}
}

impl Play for MoneyCard {
	fn is_playable(&self, _: &Assets) -> bool {
		true
	}

	fn play(self, turn: &mut Turn) {
		turn.assets.add_money(self.into());
	}
}

impl fmt::Display for MoneyCard {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}M", self.0 as u8)
	}
}
