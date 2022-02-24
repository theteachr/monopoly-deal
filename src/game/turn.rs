use super::PlayerAction;
use crate::cards::{CardKind, CardSet};
use crate::common::input;
use crate::player::Player;
use std::collections::VecDeque;

pub(crate) struct Turn {
	player: Player,
	cards_played: CardSet<CardKind>,
	cards_discarded: CardSet<CardKind>,
}

impl Turn {
	pub fn new(player: Player) -> Self {
		Self {
			player,
			cards_played: CardSet::new(),
			cards_discarded: CardSet::new(),
		}
	}

	pub fn read_action(&mut self) -> PlayerAction {
		if self.cards_played.len() == 3 {
			return PlayerAction::Pass;
		}

		self.player.print_assets();
		self.player.hand.print_numbered();

		println!("Total Bank Value: {}", self.player.played.bank_value());
		println!(
			"Total Asset Value: {}",
			self.player.played.total_property_value()
		);

		let user_input = input("> ");
		let trimmed = user_input.trim();

		if trimmed.is_empty() {
			return PlayerAction::Pass;
		}

		loop {
			match trimmed.parse::<usize>() {
				Ok(n) => break PlayerAction::Play(n),
				Err(_) => continue,
			}
		}
	}

	pub fn play(&mut self, card_position: usize, table: &mut VecDeque<Player>) {
		if let Some(card) = self.player.remove_card_at(card_position) {
			self.cards_played.add(card);
		}
	}

	pub fn terminate(mut self) -> (Player, CardSet<CardKind>, CardSet<CardKind>) {
		// A player is not allowed to have more than 7 cards in their hand at the end of a turn.
		// This needs to be checked at the end of each turn. The player should be propmted for discarding.
		let mut to_be_discarded: i8 = self.player.hand.len() as i8 - 7;

		println!("{} card(s) need to be discarded.", to_be_discarded);

		while to_be_discarded > 0 {
			self.player.hand.print_numbered();

			if let Some(card) = input("> ")
				.trim()
				.parse::<usize>()
				.ok()
				.and_then(|i| self.player.remove_card_at(i))
			{
				self.cards_discarded.add(card);
				to_be_discarded -= 1;
			}
		}

		(self.player, self.cards_played, self.cards_discarded)
	}
}
