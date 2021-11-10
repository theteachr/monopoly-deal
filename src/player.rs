use crate::cards::card::Card;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Player {
	id: usize,
	name: String,
	hand: HashSet<Card>,
}

impl Player {
	pub fn new(id: usize, name: String, hand: HashSet<Card>) -> Self {
		Player {
			id,
			name,
			hand,
		}
	}

	pub fn read(id: usize) -> Player {
		Player::new(id, String::from("Gen"), HashSet::new())
	}

	pub fn update_hand(&mut self, cards: Vec<Card>) {
		for card in cards {
			self.hand.insert(card);
		}
	}
}
