use crate::cards::{
	data::{ACTION_CARDS, MONEY_CARDS, PROPERTY_CARDS, PROPERTY_WILD_CARDS, RENT_CARDS},
	ActionCard,
	Card::{self, *},
	MoneyCard, MultiColorCard, MultiColorCardType, PropertyCard, RentVec,
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
				cards.push(Property(PropertyCard::new(
					*value,
					name,
					*color,
					RentVec::new(*color),
				)));
			}
		}

		for (value, count) in MONEY_CARDS.iter() {
			for _ in 0..*count {
				cards.push(Money(MoneyCard::new(*value)));
			}
		}

		for (value, action, count) in ACTION_CARDS.iter() {
			for _ in 0..*count {
				cards.push(Action(ActionCard::new(*value, *action)));
			}
		}

		for (value, colors, count) in PROPERTY_WILD_CARDS.iter() {
			for _ in 0..*count {
				cards.push(Wild(MultiColorCard::new(
					*value,
					*colors,
					MultiColorCardType::Property,
				)));
			}
		}

		for (value, colors, count) in RENT_CARDS.iter() {
			for _ in 0..*count {
				cards.push(Wild(MultiColorCard::new(
					*value,
					*colors,
					MultiColorCardType::Rent,
				)));
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

	pub fn len(&self) -> u8 {
		self.cards.len() as u8
	}
}
