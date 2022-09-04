use std::fmt;

use crate::cards::{BankableCardKind, Card, CardSet, PaidCardKind, PropertyCardKind, PropertySets};
use crate::color::CardColor;
use crate::common::input;

/// Holds all asset cards played by the player.
///
/// Maintains cards played as money and property cards separately.
#[derive(Debug)]
pub struct Assets {
	/// Holds cards played as money.
	pub bank: CardSet<BankableCardKind>,

	/// Holds all property cards played by the player.
	pub property_sets: PropertySets,
}

// XXX Maintain an accumulated value of all the assets to avoid calculation every time

impl Assets {
	pub fn new() -> Self {
		Self {
			bank: CardSet::new(),
			property_sets: PropertySets::new(),
		}
	}

	/// Adds the card to the `bank`.
	pub fn add_money(&mut self, card: BankableCardKind) {
		self.bank.add(card);
	}

	/// Adds a card supplied as part of payment into the assets.
	pub fn add_paid_card(&mut self, card: PaidCardKind) {
		match card {
			PaidCardKind::Banked(c) => self.add_money(c),
			PaidCardKind::Property(c) => self.add_property(c),
		}
	}

	/// Inserts the card into `property_sets`.
	pub fn add_property(&mut self, card: PropertyCardKind) {
		self.property_sets.add(card);
	}

	/// Returns the max amount of money a player can pay using the cards in their bank.
	pub fn bank_value(&self) -> u8 {
		self.bank.iter().map(Card::value).sum()
	}

	/// Returns the max amount of money a player can pay using their properties.
	pub fn total_property_value(&self) -> u8 {
		self.property_sets.total_value()
	}

	/// Returns `true` if the `amount` can be paid off of the played cards (assets).
	pub fn can_pay(&self, amount: u8) -> bool {
		self.total_property_value() + self.bank_value() >= amount
	}

	pub fn choose_banked_card(&self) -> usize {
		self.bank.choose()
	}

	pub fn choose_property_card(&self) -> usize {
		let colors: Vec<CardColor> = self.property_sets.colors().into_iter().collect();

		loop {
			match input("> ")
				.parse::<usize>()
				.ok()
				.and_then(|i| colors.get(i))
			{
				Some(color) => {
					if let Ok(i) = input("> ").parse::<usize>() {
						if let Some(cards) = self.property_sets.property_cards_of_color(*color) {
							if cards.get(i).is_some() {
								return i;
							}
						}
					}
				}
				None => continue,
			}
		}
	}

	pub fn bankrupt(&mut self) -> (Vec<BankableCardKind>, Vec<PropertyCardKind>) {
		let banked = self.bank.remove_all();
		let props = self.property_sets.go_popper();

		(banked, props)
	}
}

impl fmt::Display for Assets {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Bank: {}\nProperties: {}", self.bank, self.property_sets)
	}
}

enum AssetCardChoice {
	Bank,
	Property,
}

pub enum PaidWith {
	Bank(usize),
	Property(usize),
}

pub fn choose_card(assets: &Assets) -> PaidWith {
	let section = loop {
		// Choose automatically if one of the sections is empty.
		match (assets.bank.len(), assets.property_sets.colors().len()) {
			// Bank is empty, the user HAS to choose a property card.
			(0, _) => break AssetCardChoice::Property,

			// No property cards, the use HAS to choose a banked card.
			(_, 0) => break AssetCardChoice::Bank,

			// Both are non empty, do nothing here, let the user enter what they want to play.
			_ => {}
		};

		// Show the assets.
		println!("{}", assets);

		// TODO If the bank doesn't have enough value, automatically choose property and vice versa.

		match input("> ").as_str() {
			"b" => break AssetCardChoice::Bank,
			"p" => break AssetCardChoice::Property,
			_ => {
				println!("Use `b` to pick a banked card, or `p` for a property card.");
				continue;
			}
		}
	};

	match section {
		AssetCardChoice::Bank => PaidWith::Bank(assets.choose_banked_card()),
		AssetCardChoice::Property => PaidWith::Property(assets.choose_property_card()),
	}
}
