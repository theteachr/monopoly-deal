use crate::common::input;
use std::{collections::HashMap, fmt};

use super::Card;

/// Represents a collection of cards.
#[derive(Debug)]
pub struct CardSet<T: Card>(HashMap<usize, T>);

impl<'a, T: Card> CardSet<T> {
	/// Returns an empty collection of cards.
	pub fn new() -> Self {
		Self(HashMap::new())
	}

	/// Returns the number of cards in the collection.
	pub fn len(&self) -> usize {
		self.0.len()
	}

	/// Adds `card` into the collection.
	pub fn add(&mut self, card: T) {
		self.0.insert(card.id(), card);
	}

	/// Returns an iterator of the references of all cards in the collection.
	pub fn iter(&self) -> impl Iterator<Item = &T> {
		self.0.values()
	}

	/// Returns `true` if the collection has no cards.
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}

	pub fn get(&self, id: usize) -> Option<&T> {
		self.0.get(&id)
	}

	pub fn take(&mut self, id: usize) -> Option<T> {
		self.0.remove(&id)
	}

	/// Empties the card set and returns a `Vec` of removed cards.
	pub fn remove_all(&mut self) -> Vec<T> {
		todo!()
	}

	// XXX Track the value in a field: updated when a new card is added, and subtracted when removed
	pub fn value(&self) -> u8 {
		self.0.values().map(Card::value).sum()
	}

	pub fn choose(&self) -> usize {
		for (id, item) in self.0.iter() {
			println!("{id:03}: {item}");
		}

		loop {
			if let Some(id) = input("> ")
				.parse::<usize>()
				.ok()
				.filter(|k| self.0.contains_key(k))
			{
				return id;
			}
		}
	}
}

impl<T: Card> IntoIterator for CardSet<T> {
	type Item = T;
	type IntoIter = std::collections::hash_map::IntoValues<usize, T>;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_values()
	}
}

impl<T: fmt::Display + Card> fmt::Display for CardSet<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let string = self
			.0
			.iter()
			.map(|(_, card)| card.to_string())
			.collect::<Vec<String>>()
			.join("; ");

		write!(f, "[{}]", string)
	}
}
