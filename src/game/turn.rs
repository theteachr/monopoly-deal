use super::PlayerAction;
use crate::cards::{CardKind, CardSet, Play};
use crate::common::input;
use crate::player::{Assets, Player};

pub struct Turn {
	pub player: Player,
	pub assets: Assets,
	pub cards_discarded: CardSet<CardKind>,
	num_cards_played: u8,
}

impl Turn {
	pub fn new(player: Player, assets: Assets) -> Self {
		Self {
			player,
			assets,
			cards_discarded: CardSet::new(),
			num_cards_played: 0,
		}
	}

	pub fn read_action(&mut self) -> PlayerAction {
		if self.num_cards_played == 3 {
			return PlayerAction::Pass;
		}

		println!("{}", self.assets);
		self.player.hand.print_numbered();

		println!("Total Bank Value: {}", self.assets.bank_value());
		println!("Total Asset Value: {}", self.assets.total_property_value());

		loop {
			let user_input = input("> ");

			if user_input.is_empty() {
				return PlayerAction::Pass;
			}

			match user_input.parse::<usize>() {
				Ok(n) => break PlayerAction::Play(n),
				Err(_) => continue,
			}
		}
	}

	pub fn play(&mut self, card_position: usize) {
		if let Some(card) = self.player.remove_card_at(card_position) {
			if card.can_play(&self.assets) {
				card.play(self);
				self.num_cards_played += 1;
			} else {
				self.player.hand.add(card);
			}
		}
	}

	pub fn terminate(mut self) -> (Player, Assets, CardSet<CardKind>) {
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

		(self.player, self.assets, self.cards_discarded)
	}
}
