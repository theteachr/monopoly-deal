use crate::cards::{
	card::Card::{self, Money, Property},
	data::{MONIES, PROPERTIES},
	money_card::MoneyCard,
	property_card::PropertyCard,
	rent_vec::RentVec,
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
		let mut cards = Vec::new();

		for (value, color, titles) in PROPERTIES.iter() {
			for title in *titles {
				cards.push(Property(PropertyCard::new(
					*value,
					title,
					*color,
					RentVec::new(*color),
				)));
			}
		}

		for (value, count) in MONIES.iter() {
			for _ in 0..*count {
				cards.push(Money(MoneyCard::new(*value)));
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

		cards
	}

	pub fn len(&self) -> u8 {
		self.cards.len() as u8
	}

	pub fn add(&mut self, card: Card) {
		self.cards.push(card);
	}
}
