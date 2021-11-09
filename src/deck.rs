use crate::cards::{
	card::{
		Card,
		CardType::{Money, Property},
	},
	money_card::MoneyCard,
	property_card::PropertyCard,
	data::{MONIES, PROPERTIES},
};

use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug)]
pub struct Deck {
	cards: Vec<Card>,
}

pub enum DrawCount {
	Two,
	Five,
}

impl Deck {
	pub fn new() -> Self {
		let mut cards = Vec::new();

		for (value, color, titles, set) in PROPERTIES.iter() {
			for title in *titles {
				cards.push(Card::new(
					*value,
					Property(PropertyCard::new(title, *color, *set)),
				));
			}
		}

		for (value, count) in MONIES.iter() {
			for _ in 0..*count {
				cards.push(Card::new(*value, Money(MoneyCard)))
			}
		}

		cards.shuffle(&mut thread_rng());

		Deck { cards }
	}

	pub fn draw(&mut self, count: DrawCount) -> Vec<Card> {
		let mut cards = Vec::new();

		let count = match count {
			DrawCount::Two => 2,
			DrawCount::Five => 5,
		};

		for _ in 0..count {
			cards.push(self.cards.pop().unwrap());
		}

		cards
	}

	pub fn len(&self) -> u8 {
		self.cards.len() as u8
	}
}
