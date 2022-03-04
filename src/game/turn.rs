use super::PlayerAction;
use crate::cards::{Card, CardKind, CardSet};
use crate::common::input;
use crate::player::{Assets, Player};

/// Used to track a player's turn.
pub struct Turn {
	/// Represents the player playing the turn.
	pub player: Player,

	/// Represents the cards played by the player playing the turn.
	pub assets: Assets,

	/// Holds the cards that the player will choose to discard at the end of the turn.
	pub cards_discarded: CardSet<CardKind>,

	/// Represents the number of cards played by the player in the turn.
	num_cards_played: u8,
}

// XXX `Turn` -> `CurrentPlayer`

impl Turn {
	pub fn new(player: Player, assets: Assets) -> Self {
		Self {
			player,
			assets,
			cards_discarded: CardSet::new(),
			num_cards_played: 0,
		}
	}

	/// Returns the action the player chooses to play.
	pub fn read_action(&mut self) -> PlayerAction {
		// A player can play a max of 3 cards per turn.
		// If the number of cards played is 3, return `Pass` to indicate
		// that the turn has ended.
		if self.num_cards_played == 3 {
			return PlayerAction::Pass;
		}

		// Print the name of the player, along with the assets they own (property and money).
		println!(
			"\n<<< Assets of {} >>>\n\n{}\n",
			self.player.name, self.assets
		);

		// Print all the cards in the player's hand along with their index.
		self.player.hand.print_numbered();

		println!();

		// Print total money in the bank.
		println!("Total Bank Value: {}", self.assets.bank_value());

		// Print total value of all the properties.
		println!("Total Asset Value: {}", self.assets.total_property_value());

		println!();

		loop {
			let user_input = input("> ");

			// Let the player signal a `Pass` by just pressing the enter or return key,
			// which will result in `user_input` being empty.
			if user_input.is_empty() {
				return PlayerAction::Pass;
			}

			// Try to parse the input into a usize (index).
			match user_input.parse::<usize>() {
				// If the parsed number is a valid index, return it wrapped in `Play`.
				Ok(n) if n < self.player.hand.len() => break PlayerAction::Play(n),

				// Input couldn't be parsed, ask again.
				_ => continue,
			}
		}
	}

	/// Plays the card at index `card_position` of the player's hand if it's playable.
	///
	/// Requires: `card_position` to be a valid index pointing to a card in
	/// the player's hand.
	pub fn play(&mut self, card_position: usize) {
		// Remove the card at `card_position` from the player's hand.
		let card = self.player.remove_card_at(card_position);

		// If the card is playable, delagate the play mehcanism and increment the number of cards played,
		// else just add the card back in the player's hand.
		if card.is_playable(&self.assets) {
			card.play(self);
			self.num_cards_played += 1;
		} else {
			self.player.hand.add(card);
		}
	}

	/// Returns the mutated player, assets and a set of cards the player chose to discard.
	///
	/// A player is not allowed to have more than **7** cards in their hand at the end of a turn.
	/// This needs to be checked at the end of each turn. If there are excess cards, prompt
	/// the player to discard the excess.
	pub fn terminate(mut self) -> (Player, Assets, CardSet<CardKind>) {
		// Get the number of cards that need to be discarded.
		let mut to_be_discarded: i8 = self.player.hand.len() as i8 - 7;

		// Until the number of cards need to be discarded is > 0,
		// ask the player to enter the index of the card they want to discard.
		while to_be_discarded > 0 {
			println!("You need to discard {}.", to_be_discarded);

			println!();

			// Print the cards in hand along with their index.
			self.player.hand.print_numbered();

			println!();

			// Read the index from the user.
			let card_position = loop {
				// If the entered number can be parsed into a usize (representing a valid card index),
				// break, else ask again.
				match input("> ").parse::<usize>() {
					Ok(n) if n < self.player.hand.len() => break n,
					_ => continue,
				}
			};

			// Remove the card at `card_position` from the  player's hand,
			// and add it to the set of cards that will be dumped in the disccard pile.
			self.cards_discarded
				.add(self.player.remove_card_at(card_position));
			to_be_discarded -= 1;
		}

		(self.player, self.assets, self.cards_discarded)
	}
}
