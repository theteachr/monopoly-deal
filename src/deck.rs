use crate::cards::{
	data::{ACTION_CARDS, MONEY_CARDS, PROPERTY_CARDS, PROPERTY_WILD_CARDS, RENT_CARDS},
	Card, RentVec,
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

		for (value, color, names) in PROPERTY_CARDS.iter() {
			for name in *names {
				cards.push(Card::new_property_card(
					*value,
					name,
					*color,
					RentVec::new(*color),
				));
			}
		}

		for (value, count) in MONEY_CARDS.iter() {
			for _ in 0..*count {
				cards.push(Card::new_money_card(*value));
			}
		}

		for (value, action, count) in ACTION_CARDS.iter() {
			for _ in 0..*count {
				cards.push(Card::new_action_card(*value, *action));
			}
		}

		for (value, colors, count) in PROPERTY_WILD_CARDS.iter() {
			for _ in 0..*count {
				cards.push(Card::new_property_wild_card(*value, *colors));
			}
		}

		for (value, colors, count) in RENT_CARDS.iter() {
			for _ in 0..*count {
				cards.push(Card::new_rent_card(*value, *colors));
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
