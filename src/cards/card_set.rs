use crate::cards::card::Card;

#[derive(Debug)]
pub struct CardSet {
	cards: Vec<Card>,
	size: usize,
}

impl CardSet {
	pub fn new() -> Self {
		Self {
			cards: Vec::with_capacity(7),
			size: 0,
		}
	}

	pub fn len(&self) -> usize {
		self.size
	}

	pub fn add(&mut self, card: Card) {
		self.cards.push(card);
		self.size += 1;
	}

	pub fn cards(&self) -> Vec<&Card> {
		self.cards.iter().collect()
	}

	pub fn remove(&mut self, position: usize) -> Card {
		// swap card at `postion` with the last card, then pop
		self.cards.swap(position, self.size - 1);
		self.size -= 1;

		return self.cards.pop().unwrap();
	}
}
