use crate::cards::{card::Card, card_set::CardSet};

#[derive(Debug)]
pub struct Player {
	pub id: usize,
	pub name: String,
	pub hand: CardSet,
}

impl Player {
	pub fn new(id: usize, name: String) -> Self {
		Self {
			id,
			name,
			hand: CardSet::new(),
		}
	}

	pub fn read(id: usize) -> Player {
		Player::new(id, String::from("Gen"))
	}

	pub fn update_hand(&mut self, cards: Vec<Card>) {
		println!("Adding {:?} to the hand....", cards);
		for card in cards {
			self.hand.add(card);
		}
	}

	pub fn cards_in_hand(&self) -> Vec<&Card> {
		self.hand.cards()
	}
}
