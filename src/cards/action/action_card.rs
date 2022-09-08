use super::play_fns::*;
use crate::cards::{Card, PropertySets};
use crate::entities::CurrentPlayer;
use crate::errors::NotPlayable;
use crate::Game;

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
	pub id: usize,
	pub value: u8,
	pub action: Action,
}

impl ActionCard {
	pub fn new(id: usize, value: u8, action: Action) -> Self {
		Self { id, value, action }
	}

	pub fn is_playable(&self, _properties: &PropertySets) -> Result<(), NotPlayable> {
		match self.action {
			Action::PassGo | Action::Birthday | Action::DebtCollector => Ok(()),
			Action::DoubleTheRent => Err(NotPlayable(
				"You need to play a rent card before playing this one.".to_string(),
			)),
			Action::DealBreaker => Err(NotPlayable("Not implemented yet.".to_string())),
			Action::ForcedDeal => Err(NotPlayable("Not implemented yet.".to_string())),
			Action::Hotel => Err(NotPlayable("Not implemented yet.".to_string())),
			Action::House => Err(NotPlayable("Not implemented yet.".to_string())),
			Action::JustSayNo => Err(NotPlayable(
				"Can only be played when you're asked to pay or to counter another JustSayNo."
					.to_string(),
			)),
			Action::SlyDeal => Ok(()),
		}
	}

	pub fn play(self, player: &mut CurrentPlayer, game: &mut Game) {
		// TODO Allow for playing `ActionCard`s as money.
		match self.action {
			Action::PassGo => play_pass_go(player.get(), &mut game.deck),
			_ => todo!(),
		}

		game.discard_deck.push_back(self.into());
	}
}

impl Card for ActionCard {
	fn value(&self) -> u8 {
		self.value
	}

	fn id(&self) -> usize {
		self.id
	}
}

impl fmt::Display for ActionCard {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.action.fmt(f)
	}
}
