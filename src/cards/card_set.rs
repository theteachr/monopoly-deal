use std::{fmt, ops::Index};

use super::Card;

/// Represents a collection of cards.
#[derive(Debug)]
pub struct CardSet<T> {
	cards: Vec<T>,
	size: usize,
}

impl<T: fmt::Display + Card> CardSet<T> {
	/// Returns an empty collection of cards.
	pub fn new() -> Self {
		Self {
			cards: Vec::new(),
			size: 0,
		}
	}

	/// Returns the number of cards in the collection.
	pub fn len(&self) -> usize {
		self.size
	}

	/// Adds `card` into the collection.
	pub fn add(&mut self, card: T) {
		self.cards.push(card);
		self.size += 1;
	}

	/// Returns an iterator of the references of all cards in the collection.
	pub fn iter(&self) -> impl Iterator<Item = &T> {
		self.cards.iter()
	}

	/// Returns `true` if the collection has no cards.
	pub fn is_empty(&self) -> bool {
		self.size == 0
	}

	/// Returns the card present at index = `position`.
	/// Panics if the index in not valid.
	pub fn remove(&mut self, position: usize) -> T {
		let removed = self.cards.swap_remove(position);
		self.size -= 1;

		removed
	}

	pub fn value(&self) -> u8 {
		self.cards.iter().map(Card::value).sum()
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
