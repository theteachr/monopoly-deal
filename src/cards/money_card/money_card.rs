use super::denomination::Denomination;
use crate::cards::Card;
use crate::player::Player;
use std::{cmp::PartialEq, fmt, hash::Hash};

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct MoneyCard(Denomination);

impl MoneyCard {
	pub fn new(value: u8) -> Self {
		Self(value.into())
	}

	pub fn play(self, player: &mut Player) -> Option<u8> {
		player.add_money(self.into());

		Some(5)
	}
}

impl Card for MoneyCard {
	fn value(&self) -> u8 {
		self.0 as u8
	}
}

impl fmt::Display for MoneyCard {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}M", self.0 as u8)
	}
}
