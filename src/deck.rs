use crate::cards::{
	data::{ACTION_CARDS, MONEY_CARDS, PROPERTY_CARDS, PROPERTY_WILD_CARDS, RENT_CARDS},
	ActionCard, Card, MoneyCard, PropertyCard, PropertyWildCard, RentCard,
	property_card::RentVec,
};

use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug)]
pub struct Deck {
	cards: Vec<Card>,
}

#[repr(u8)]
pub enum DrawCount {
	Two = 2,
	Five = 5,
}

impl Deck {
	pub fn new() -> Self {
		let mut cards = Vec::<Card>::new();

		for (value, color, names) in PROPERTY_CARDS.iter() {
			for name in *names {
				cards.push(PropertyCard::new(*value, name, *color, RentVec::new(*color)).into());
			}
		}

		for (value, count) in MONEY_CARDS.iter() {
			for _ in 0..*count {
				cards.push(MoneyCard::new(*value).into());
			}
		}

		for (value, action, count) in ACTION_CARDS.iter() {
			for _ in 0..*count {
				cards.push(ActionCard::new(*value, *action).into());
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

		Self { cards }
	}

	pub fn draw(&mut self, count: DrawCount) -> Vec<Card> {
		let mut cards = Vec::new();

		for _ in 0..count as u8 {
			cards.push(self.cards.pop().unwrap());
		}

		return cards;
	}

	pub fn push_back(&mut self, card: Card) {
		self.cards.push(card);
	}

	pub fn len(&self) -> u8 {
		self.cards.len() as u8
	}
}
