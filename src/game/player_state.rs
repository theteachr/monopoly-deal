use std::collections::HashSet;

use crate::cards::card::Card;

#[derive(Debug)]
pub struct PlayerState {
	name: String,
	cards_played: HashSet<Card>,
}

impl PlayerState {
	pub fn new(name: String) -> Self {
		Self {
			name,
			cards_played: HashSet::new(),
		}
	}

	pub fn cards(&self) -> Vec<&Card> {
		self.cards_played.iter().collect()
	}
}
