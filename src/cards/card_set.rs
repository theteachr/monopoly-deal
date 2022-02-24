use std::{fmt, ops::Index};

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

	pub fn remove(&mut self, position: usize) -> Option<T> {
		if position >= self.size {
			return None;
		}

		let removed = self.cards.swap_remove(position);
		self.size -= 1;

		Some(removed)
	}

    pub fn card_at(&self, index: usize) -> Option<&T> {
        if index >= self.size {
            return None;
        }

        Some(&self.cards[index])
    }

	pub fn print_numbered(&self) {
		for (i, card) in self.cards.iter().enumerate() {
			println!("{}: {}", i, card);
		}
	}
}

impl<T> Index<usize> for CardSet<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.cards[index]
    }
}

impl<T> Iterator for CardSet<T> {
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		self.cards.pop()
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
