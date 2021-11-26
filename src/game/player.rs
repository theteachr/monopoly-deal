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

	pub fn play_card_at(&mut self, position: usize) {
		let selected_card = self.hand.remove(position);
		self.played.add(selected_card);
	}

    pub fn play_cards_at(&mut self, mut card_positions: Vec<u8>) {
        card_positions.sort_by_key(|k| std::cmp::Reverse(*k));

        for pos in card_positions {
            self.play_card_at(pos.into());
        }
    }
}
