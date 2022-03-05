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
	pub hand: CardSet<CardKind>,
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
		let draw_count = match self.hand.is_empty() {
			true => DrawCount::Five,
			false => DrawCount::Two,
		};

		// Draw the cards from the deck and add them to the player's hand.
		for card in deck.draw(draw_count) {
			self.hand.add(card);
		}
	}

	/// Returns the card present at `index`.
	/// 
	/// **Precondition**: `index` should not be out of bounds, panics otherwise.
	pub fn remove_card_at(&mut self, index: usize) -> CardKind {
		return self.hand.remove(index);
	}
}
