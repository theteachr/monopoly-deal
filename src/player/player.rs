use crate::cards::{CardKind, CardSet};
use crate::deck::{Deck, DrawCount};

/// Represents a player.
#[derive(Debug)]
pub struct Player {
	///
	pub id: usize,

	/// Holds the name of the the player.
	pub name: String,

	/// Contains cards the player is holding in their hand.
	hand: CardSet<CardKind>,
}

impl Player {
	/// Returns a player with an empty hand.
	pub fn new(id: usize, name: String) -> Self {
		Self {
			id,
			name,
			hand: CardSet::new(),
		}
	}

	/// Draws cards from the `deck` and puts them in the player's `hand`.
	///
	/// #### Arguments
	///
	/// * `deck` - a mutable reference to the game's deck
	pub fn draw(&mut self, deck: &mut Deck) {
		// If the hand is empty, draw 5, else 2.
		self.draw_cards(
			deck,
			match self.hand.is_empty() {
				true => DrawCount::Five,
				false => DrawCount::Two,
			},
		)
	}

	/// Draws two cards from the deck and adds them into the player's hand.
	pub fn draw_two(&mut self, deck: &mut Deck) {
		self.draw_cards(deck, DrawCount::Two)
	}

	/// Draws `n` (2 or 5) cards from the deck then adds them into the player's hand.
	fn draw_cards(&mut self, deck: &mut Deck, n: DrawCount) {
		deck.draw(n)
			.into_iter()
			.for_each(|card| self.hand.add(card))
	}

	pub fn hand_iter(&self) -> impl Iterator<Item = &CardKind> {
		self.hand.iter()
	}

	pub fn hand_len(&self) -> usize {
		self.hand.len()
	}

	pub fn card_at(&self, index: usize) -> Option<&CardKind> {
		self.hand.get(index)
	}

	/// Returns the card present at `index`.
	///
	/// **Panics** if `index` is out of bounds.
	pub fn remove_card_at(&mut self, index: usize) -> CardKind {
		self.hand.remove(index)
	}
}
