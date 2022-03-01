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

		println!(
			"\n<<< Assets of {} >>>\n\n{}\n",
			self.player.name, self.assets
		);

		self.player.hand.print_numbered();

		println!();
		println!("Total Bank Value: {}", self.assets.bank_value());
		println!("Total Asset Value: {}", self.assets.total_property_value());
		println!();

		loop {
			let user_input = input("> ");

			if user_input.is_empty() {
				return PlayerAction::Pass;
			}

			match user_input.parse::<usize>() {
				Ok(n) if n < self.player.hand.len() => break PlayerAction::Play(n),
				_ => continue,
			}
		}
	}

	pub fn play(&mut self, card_position: usize) {
		// FIXME
		let card = self.player.remove_card_at(card_position);

		if card.is_playable(&self.assets) {
			card.play(self);
			self.num_cards_played += 1;
		} else {
			self.player.hand.add(card);
		}
	}

	pub fn terminate(mut self) -> (Player, Assets, CardSet<CardKind>) {
		// A player is not allowed to have more than 7 cards in their hand at the end of a turn.
		// This needs to be checked at the end of each turn. The player should be propmted for discarding.
		let mut to_be_discarded: i8 = self.player.hand.len() as i8 - 7;

		while to_be_discarded > 0 {
			println!("You need to discard {}.", to_be_discarded);

			println!();
			self.player.hand.print_numbered();
			println!();

			let card_position = loop {
				match input("> ").parse::<usize>() {
					Ok(n) if n < self.player.hand.len() => break n,
					_ => continue,
				}
			};

			self.cards_discarded
				.add(self.player.remove_card_at(card_position));
			to_be_discarded -= 1;
		}

		(self.player, self.assets, self.cards_discarded)
	}
}
