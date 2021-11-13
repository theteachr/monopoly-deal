use crate::cards::card::Card;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Player {
	pub id: usize,
	pub name: String,
	pub hand: HashSet<Card>,
}

impl Player {
	pub fn new(id: usize, name: String) -> Self {
		Self {
			id,
			name,
			hand: HashSet::new(),
		}
	}

	pub fn read(id: usize) -> Player {
		Player::new(id, String::from("Gen"))
	}

	pub fn update_hand(&mut self, cards: Vec<Card>) {
		for card in cards {
			self.hand.insert(card);
		}
	}

	pub fn cards_in_hand(&self) -> Vec<&Card> {
		self.hand.iter().collect()
	}
}