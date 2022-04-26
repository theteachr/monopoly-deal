use std::fmt;

use crate::cards::{BankableCardKind, Card, CardSet, PaidCardKind, PropertyCardKind, PropertySets};
use crate::color::CardColor;
use crate::common::{input, print_read_index};

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

	/// Removes a card from the bank.
	pub fn remove_banked_card(&mut self, idx: usize) -> BankableCardKind {
		self.bank.remove(idx)
	}

	/// Removes a `color` colored card from the property sets played by the player.
	pub fn remove_property_card_of_color(&mut self, color: &CardColor) -> PropertyCardKind {
		self.property_sets.pop(color)
	}

	/// Returns `true` if the `amount` can be paid off of the played cards (assets).
	pub fn can_pay(&self, amount: u8) -> bool {
		self.total_property_value() + self.bank_value() >= amount
	}

	pub fn bankrupt(&mut self) -> CardSet<PaidCardKind> {
		// XXX impl FromIterator for CardSet
		let mut cards = CardSet::new();

		self.bank
			.remove_all()
			.into_iter()
			.map(PaidCardKind::from)
			.chain(self.property_sets.go_popper().into_iter())
			.for_each(|card| cards.add(card));

		cards
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

pub enum PayWith {
	Bank(usize),
	Property(CardColor),
}

pub fn choose_card(assets: &Assets) -> PayWith {
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
		AssetCardChoice::Bank => {
			let idx = print_read_index("$ ", assets.bank.iter(), assets.bank.len());

			PayWith::Bank(idx)
		}
		AssetCardChoice::Property => {
			let colors = assets.property_sets.iter().collect::<Vec<_>>();
			let idx = print_read_index("# ", colors.iter(), colors.len());

			PayWith::Property(colors[idx])
		}
	}
}
