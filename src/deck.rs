use crate::cards::{
	data::{ACTION_CARDS, MONEY_CARDS, PROPERTY_CARDS, PROPERTY_WILD_CARDS, RENT_CARDS},
	ActionCardKind, CardKind, MoneyCard, PropertyCard, PropertyWildCard, RentCard,
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

impl Deck {
	/// Returns a shuffled deck of all cards in the game.
	pub fn new() -> Self {
		let mut cards: Vec<CardKind> = Vec::new();

		for (color, names) in PROPERTY_CARDS.iter() {
			for name in *names {
				cards.push(PropertyCard::new(name, *color).into());
			}
		}

		for (value, count) in MONEY_CARDS.iter() {
			for _ in 0..*count {
				cards.push(MoneyCard::new(*value).into());
			}
		}

		for (action, count) in ACTION_CARDS.iter() {
			for _ in 0..*count {
				cards.push(ActionCardKind::from(*action).into());
			}
		}

		for (value, colors, count) in PROPERTY_WILD_CARDS.iter() {
			for _ in 0..*count {
				cards.push(PropertyWildCard::new(*value, *colors).into());
			}
		}

		for (value, colors, count) in RENT_CARDS.iter() {
			for _ in 0..*count {
				cards.push(RentCard::new(*value, *colors).into());
			}
		}

		cards.shuffle(&mut thread_rng());

		Self(cards)
	}

	/// Returns the top `count` cards of the deck.
	pub fn draw(&mut self, count: DrawCount) -> Vec<CardKind> {
		let mut cards = Vec::new();

		for _ in 0..count as u8 {
			cards.push(self.0.pop().unwrap());
		}

		return cards;
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
