use crate::cards::{card::Card, card_set::CardSet};

#[derive(Debug)]
pub struct Player {
	pub id: usize,
	pub name: String,
	pub hand: CardSet,
	pub played: CardSet,
}

impl Player {
	pub fn new(id: usize, name: String) -> Self {
		Self {
			id,
			name,
			hand: CardSet::new(),
			played: CardSet::new(),
		}
	}

	pub fn update_hand(&mut self, cards: Vec<Card>) {
		for card in cards {
			self.hand.add(card);
		}
	}

	pub fn hand(&self) -> Vec<&Card> {
		self.hand.cards()
	}

	pub fn played(&self) -> Vec<&Card> {
		self.played.cards()
	}
}
