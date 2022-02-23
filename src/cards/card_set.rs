use std::fmt;

#[derive(Debug)]
pub struct CardSet<T> {
	cards: Vec<T>,
	size: usize,
}

impl<T: fmt::Display> CardSet<T> {
	pub fn new() -> Self {
		Self {
			cards: Vec::new(),
			size: 0,
		}
	}

	pub fn len(&self) -> usize {
		self.size
	}

	pub fn add(&mut self, card: T) {
		self.cards.push(card);
		self.size += 1;
	}

	pub fn iter(&self) -> impl Iterator<Item = &T> {
		self.cards.iter()
	}

	pub fn is_empty(&self) -> bool {
		self.size == 0
	}

	pub fn cards(&self) -> Vec<&T> {
		self.cards.iter().collect()
	}

	pub fn remove(&mut self, position: usize) -> Option<T> {
		if position >= self.size {
			return None;
		}

		let removed = self.cards.swap_remove(position);
		self.size -= 1;

		return Some(removed);
	}

    pub fn print_numbered(&self) {
		for (i, card) in self.cards.iter().enumerate() {
			println!("{}: {}", i, card);
		}
    }
}

impl<T: fmt::Display> fmt::Display for CardSet<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let string = self
			.cards
			.iter()
			.map(|card| card.to_string())
			.collect::<Vec<String>>()
			.join("; ");

		write!(f, "[{}]", string)
	}
}
