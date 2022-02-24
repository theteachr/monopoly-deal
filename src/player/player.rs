use crate::cards::{BankableCardKind, CardKind, CardSet, Play, PropertyCardKind};
use crate::color::CardColor;
use crate::deck::{Deck, DrawCount};
use crate::player::Assets;

#[derive(Debug)]
pub struct Player {
	pub id: usize,
	pub name: String,
	pub hand: CardSet<CardKind>,
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

	pub fn draw(&mut self, deck: &mut Deck) {
		let draw_count = match self.hand.is_empty() {
			true => DrawCount::Five,
			false => DrawCount::Two,
		};

		for card in deck.draw(draw_count) {
			self.hand.add(card);
		}
	}

	pub fn add_money(&mut self, card: BankableCardKind) {
		self.played.add_money(card);
	}

	pub fn add_property(&mut self, card: PropertyCardKind) {
		self.played.add_property(card);
	}

	pub fn remove_card_at(&mut self, card_position: usize) -> Option<CardKind> {
		if let Some(card) = self.hand.card_at(card_position) {
			if card.can_play(&self) {
				return self.hand.remove(card_position);
			}
		}

        None
	}

	pub fn owns_asset_of_color(&self, color: CardColor) -> bool {
		self.played.property_sets.exists(color)
	}

	pub fn rent(&self, color: CardColor) -> u8 {
		self.played.rent(color)
	}

	pub fn print_assets(&self) {
		println!("{}'s assets: {}", self.name, self.played);
	}
}
