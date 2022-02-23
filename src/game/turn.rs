use super::PlayerAction;
use crate::player::Player;
use crate::common::input;
use std::collections::VecDeque;
use crate::deck::Deck;

pub(crate) struct Turn {
	player: Player,
	num_cards_played: u8,
}

impl Turn {
	pub fn new(player: Player) -> Self {
		Self {
			player,
			num_cards_played: 0,
		}
	}

	pub fn read_action(&mut self) -> PlayerAction {
		if self.num_cards_played == 3 {
			return PlayerAction::Pass;
		}

		self.player.print_assets();
		self.player.hand.print_numbered();

		let user_input = input("> ");
		let trimmed = user_input.trim();

		if trimmed.is_empty() {
			return PlayerAction::Pass;
		}

		loop {
			match trimmed.parse::<u8>() {
				Ok(n) => break PlayerAction::Play(n),
				Err(_) => continue,
			}
		}
	}

	pub fn play(&mut self, card_position: u8, table: &mut VecDeque<Player>) {
		if let Some(n) = self
			.player
			.remove_card_at(card_position)
			.and_then(|card| card.play(table, &mut self.player))
		{
			println!("{}", n);
			self.num_cards_played += 1;
		}
	}

	pub fn terminate(mut self, table: &mut VecDeque<Player>, discard_pile: &mut Deck) {
		// A player is not allowed to have more than 7 cards in their hand at the end of a turn.
		// This needs to be checked at the end of each turn. The player should be propmted for discarding.
		let mut to_be_discarded: i8 = self.player.hand.len() as i8 - 7;

		println!("{} card(s) need to be discarded.", to_be_discarded);

		while to_be_discarded > 0 {
			self.player.hand.print_numbered();

			if let Some(card) = input("> ")
				.trim()
				.parse::<u8>()
				.ok()
				.and_then(|i| self.player.remove_card_at(i))
			{
				discard_pile.push_back(card);
				to_be_discarded -= 1;
			}
		}

		table.push_back(self.player);
	}
}
