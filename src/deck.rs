use crate::cards::{
	data::{ACTION_CARDS, MONEY_CARDS, PROPERTY_CARDS, PROPERTY_WILD_CARDS, RENT_CARDS},
	ActionCard, CardKind, MoneyCard, PropertyCard, PropertyWildCard, RentCard,
};

use rand::seq::SliceRandom;
use rand::thread_rng;

/// Represents the deck of cards.
#[derive(Debug)]
pub struct Deck(Vec<CardKind>);

/// Represents the count of cards that can be drawn from the deck.
#[repr(u8)]
pub enum DrawCount {
	Two = 2,
	Five = 5,
}

struct IdGen(usize);

impl IdGen {
	fn new() -> Self {
		Self(0)
	}
}

impl Iterator for IdGen {
	type Item = usize;

	fn next(&mut self) -> Option<Self::Item> {
		self.0 += 1;
		Some(self.0)
	}
}

impl Default for Deck {
	/// Returns a shuffled deck of all cards in the game.
    fn default() -> Self {
		let mut cards: Vec<CardKind> = Vec::new();
		let mut ids = IdGen::new();

		for (color, names) in PROPERTY_CARDS.iter() {
			for name in *names {
				cards.push(PropertyCard::new(ids.next().unwrap(), name, *color).into());
			}
		}

		for (value, count) in MONEY_CARDS.iter() {
			for _ in 0..*count {
				cards.push(MoneyCard::new(ids.next().unwrap(), (*value).into()).into());
			}
		}

		for (value, action, count) in ACTION_CARDS.iter() {
			for _ in 0..*count {
				cards.push(ActionCard::new(ids.next().unwrap(), *value, *action).into());
			}
		}

		for (value, colors, count) in PROPERTY_WILD_CARDS.iter() {
			for _ in 0..*count {
				cards.push(PropertyWildCard::new(ids.next().unwrap(), *value, *colors).into());
			}
		}

		for (value, colors, count) in RENT_CARDS.iter() {
			for _ in 0..*count {
				cards.push(RentCard::new(ids.next().unwrap(), *value, *colors).into());
			}
		}

		cards.shuffle(&mut thread_rng());

		Self(cards)
    }
}

impl Deck {
	pub fn new() -> Self {
        Self(Vec::new())
	}

	/// Returns the top `count` cards of the deck.
	pub fn draw(&mut self, count: DrawCount) -> Vec<CardKind> {
		let mut cards = Vec::new();

		for _ in 0..count as u8 {
			cards.push(self.0.pop().unwrap());
		}

		cards
	}

	/// Adds `card` into the deck.
	pub fn push_back(&mut self, card: CardKind) {
		self.0.push(card);
	}

	/// Returns the number of cards in the deck.
	pub fn len(&self) -> u8 {
		self.0.len() as u8
	}
}
