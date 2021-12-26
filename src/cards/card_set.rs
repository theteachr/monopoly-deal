use std::fmt;

#[derive(Debug)]
pub struct CardSet<T> {
	cards: Vec<T>,
	size: usize,
}

// XXX: Implement `Iterator`?

impl<T> CardSet<T> {
	pub fn new() -> Self {
		Self {
			cards: Vec::new(),
			size: 0,
		}
	}

	pub fn len(&self) -> u8 {
		self.size as u8
	}

	pub fn add(&mut self, card: T) {
		self.cards.push(card);
		self.size += 1;
	}

	pub fn cards(&self) -> Vec<&T> {
		self.cards.iter().collect()
	}

	pub fn remove(&mut self, position: usize) -> T {
		// swap card at `postion` with the last card, then pop
		self.cards.swap(position, self.size - 1);
		self.size -= 1;

		return self.cards.pop().unwrap();
	}
}

impl<T: fmt::Display> fmt::Display for CardSet<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let string = self
			.cards
			.iter()
			.map(|card| card.to_string())
			.collect::<Vec<String>>()
			.join(" ; ");

		write!(f, "[ {} ]", string)
	}
}
