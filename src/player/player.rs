use crate::cards::{CardKind, CardSet};
use crate::deck::{Deck, DrawCount};

#[derive(Debug)]
pub struct Player {
	pub id: usize,
	pub name: String,
	pub hand: CardSet<CardKind>,
}

impl Player {
	pub fn new(id: usize, name: String) -> Self {
		Self {
			id,
			name,
			hand: CardSet::new(),
		}
	}

	pub fn draw(&mut self, deck: &mut Deck) {
		let draw_count = match self.hand.is_empty() {
			true => DrawCount::Five,
			false => DrawCount::Two,
		};

		for card in deck.draw(draw_count) {
			self.hand.add(card);
		}
	}

	pub fn remove_card_at(&mut self, card_position: usize) -> Option<CardKind> {
		return self.hand.remove(card_position);
	}
}
