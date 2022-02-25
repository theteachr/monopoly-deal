use super::Action;
use crate::cards::{Card, Play};
use crate::game::Turn;
use crate::player::Assets;
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
}

impl Card for ActionCard {
	fn value(&self) -> u8 {
		self.value
	}
}

impl Play for ActionCard {
	fn can_play(&self, _turn: &Assets) -> bool {
		false
	}

	fn play(self, _turn: &mut Turn) {
		// TODO Ask whether to bank it or play it
		println!("Implementing `Playable` for ActionCard...");
	}
}

impl fmt::Display for ActionCard {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.action.fmt(f)
	}
}
