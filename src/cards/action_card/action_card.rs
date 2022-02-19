use super::Action;
use crate::cards::Card;
use crate::player::Player;
use std::collections::VecDeque;
use std::{cmp::PartialEq, fmt, hash::Hash};

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct ActionCard {
	value: u8,
	action: Action,
}

impl ActionCard {
	pub fn new(value: u8, action: Action) -> Self {
		Self { value, action }
	}

	pub fn play(self, table: &mut VecDeque<Player>, _player: &mut Player) {
		// TODO Ask whether to bank it or play it
		println!("Implementing `Playable` for ActionCard...");
	}
}

impl Card for ActionCard {
	fn value(&self) -> u8 {
		self.value
	}
}

impl fmt::Display for ActionCard {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.action.fmt(f)
	}
}
