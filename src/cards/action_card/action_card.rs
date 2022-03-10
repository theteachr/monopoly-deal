use crate::cards::{Card, PropertySets};
use crate::deck::Deck;
use crate::errors::NotPlayable;
use crate::game::{CurrentPlayer, Game};
use crate::player::Player;
use std::fmt::Debug;
use std::{cmp::PartialEq, fmt, hash::Hash};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum Action {
	Birthday,
	DealBreaker,
	DebtCollector,
	DoubleTheRent,
	ForcedDeal,
	Hotel,
	House,
	JustSayNo,
	PassGo,
	SlyDeal,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct ActionCard {
	value: u8,
	action: Action,
}

impl ActionCard {
	pub fn new(value: u8, action: Action) -> Self {
		Self {
			value,
			action: action.into(),
		}
	}

	pub fn play(self, player: &mut CurrentPlayer, game: &mut Game) {
		match self.action {
			Action::PassGo => play_pass_go(player.get(), &mut game.deck),
			_ => todo!(),
		}

		game.discard_deck.push_back(self.into());
	}
}

/// Draws two cards from the deck and adds them into the player's hand.
fn play_pass_go(player: &mut Player, deck: &mut Deck) {
	player.draw_two(deck);
}

impl Card for ActionCard {
	fn value(&self) -> u8 {
		self.value
	}

	fn is_playable(&self, _properties: &PropertySets) -> Result<(), NotPlayable> {
		match self.action {
			Action::PassGo => Ok(()),
			_ => Err(NotPlayable(format!(
				"Action for {} is not implemented yet...",
				self
			))),
		}
	}
}

impl fmt::Display for ActionCard {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self.action)
	}
}
