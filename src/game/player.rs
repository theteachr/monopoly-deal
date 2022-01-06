use crate::cards::{BankableCardKind, Card, CardSet, PropertyCardKind};
use crate::game::Playable;

use std::fmt;

#[derive(Debug)]
pub struct Assets {
	bank: CardSet<BankableCardKind>,
	props: CardSet<PropertyCardKind>,
}

impl fmt::Display for Assets {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Bank: {} Properties: {}", self.bank, self.props)
	}
}

impl Assets {
	pub fn new() -> Self {
		Self {
			bank: CardSet::new(),
			props: CardSet::new(),
		}
	}

	pub fn add_money(&mut self, card: BankableCardKind) {
		self.bank.add(card);
	}

	pub fn add_property(&mut self, card: PropertyCardKind) {
		self.props.add(card);
	}
}

#[derive(Debug)]
pub struct Player {
	pub id: usize,
	pub name: String,
	pub hand: CardSet<Card>,
	pub played: Assets,
}

impl Player {
	pub fn new(id: usize, name: String) -> Self {
		Self {
			id,
			name,
			hand: CardSet::new(),
			played: Assets::new(),
		}
	}

	pub fn update_hand(&mut self, cards: Vec<Card>) {
		for card in cards {
			self.hand.add(card);
		}
	}

	pub fn add_money(&mut self, card: BankableCardKind) {
		self.played.add_money(card);
	}

	pub fn add_property(&mut self, card: PropertyCardKind) {
		self.played.add_property(card);
	}

	pub fn play(&mut self, card: Card) {
		card.play(self);
	}

	pub fn print_assets(&self) {
		println!("{}'s assets: {}", self.name, self.played);
	}

	pub fn print_numbered_hand(&self) {
		println!("{}'s hand:", self.name);

		for (i, card) in self.hand.cards().iter().enumerate() {
			println!("{}: {}", i, card);
		}
	}
}
