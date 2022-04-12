use super::PlayerAction;
use crate::cards::{Action, Card, CardKind, CardSet};
use crate::common::{input, print_indexed, read_index};
use crate::errors::Failed;
use crate::player::{Assets, Player};

/// Stores all necessary information about the player playing the current turn.
pub struct CurrentPlayer {
	/// Represents the player playing the turn.
	pub player: Player,

	/// Represents the cards played by the player playing the turn.
	pub assets: Assets,

	/// Holds the cards that the player will choose to discard at the end of the turn.
	pub cards_discarded: CardSet<CardKind>,

	/// Represents the number of cards played by the player in the turn.
	pub num_cards_played: u8,
}

impl CurrentPlayer {
	pub fn new(player: Player, assets: Assets) -> Self {
		Self {
			player,
			assets,
			cards_discarded: CardSet::new(),
			num_cards_played: 0,
		}
	}

	/// Returns a mutable reference to the player playing the turn.
	pub fn get(&mut self) -> &mut Player {
		&mut self.player
	}

	/// Returns the action the player chooses to play.
	pub fn read_action(&mut self) -> PlayerAction {
		// A player can play a max of 3 cards per turn.
		// If the number of cards played is 3, return `Pass` to indicate
		// that the turn has ended.
		if self.num_cards_played == 3 {
			return PlayerAction::Pass;
		}

		// Show the name of the player, along with the assets they own (property and money).
		println!(
			"\n<<< Assets of {} >>>\n\n{}\n",
			self.player.name, self.assets
		);

		// Show all the cards in the player's hand along with their index.
		print_indexed(self.player.hand_iter());

		// Show total money in the bank.
		println!("Total Bank Value: {}", self.assets.bank_value());

		// Show total value of all the properties.
		println!("Total Asset Value: {}", self.assets.total_property_value());

		loop {
			// Read the input from the user.
			let user_input = input("> ");

			// Let the player signal a `Pass` by just pressing the enter or the return key.
			if user_input.is_empty() {
				break PlayerAction::Pass;
			}

			let parsed = user_input.parse::<usize>();

			// If `user_input` can be parsed into a valid index, and the card at it `is_playable`,
			// remove the card from hand, and return it wrapped inside `Play`.
			match parsed
				.clone()
				.map_err(Failed::from)
				.and_then(|n| self.player.card_at(n).ok_or(Failed::InvalidIndex(n)))
				.and_then(|card| self.can_play(card))
			{
				Ok(_) => {
					// At this point, we know that this card will be played,
					// so incement the number of cards played by the player.
					self.num_cards_played += 1;

					// Since `parsed` was checked to see if it were a valid index,
					// we're safe to `unwrap` it.
					// Remove the card out of the player's hand and return it wrapped in `Play`.
					break PlayerAction::Play(self.player.remove_card_at(parsed.unwrap()));
				}
				Err(e) => println!("{}", e),
			}
		}
	}

	// Returns an `Ok(())` if the player can play the `card` in this turn.
	fn can_play(&self, card: &CardKind) -> Result<(), Failed> {
		// `DoubleTheRent` needs to be handled separately as it's the only card
		// that needs to know the cards the player has played in the turn to
		// determine if it's playable.
		if let CardKind::ActionCard(card) = card {
			if matches!(card.action, Action::DoubleTheRent) {
				// If the player has played a rent card, `DoubleTheRent` card
				// is playable.
				return Err(Failed::NotPlayable(
					"DoubleTheRent is tricky...".to_string(),
				));
			}
		}

		card.is_playable(&self.assets.property_sets)
			.map_err(Failed::from)
	}

	/// Returns the mutated player, assets and a set of cards the player chose to discard.
	///
	/// A player is not allowed to have more than **7** cards in their hand at the end of a turn.
	/// This needs to be checked at the end of each turn. If there are excess cards, prompt
	/// the player to discard the excess.
	pub fn end_turn(mut self) -> (Player, Assets, CardSet<CardKind>) {
		// Get the number of cards that need to be discarded.
		let mut to_be_discarded: i8 = self.player.hand_len() as i8 - 7;

		// Until the number of cards need to be discarded is > 0,
		// ask the player to enter the index of the card they want to discard,
		// and add the card into the discard pile.
		while to_be_discarded > 0 {
			println!("You need to discard {}.", to_be_discarded);

			// Show the cards and read the index.
			let idx = print_read_index("> ", self.player.hand_iter(), self.player.hand_len());

			// Remove the card at `idx` from the  player's hand,
			// and add it to the set of cards that will be dumped in the discard pile.
			self.cards_discarded.add(self.player.remove_card_at(idx));

			// One less card to be discarded, decrement the variable.
			to_be_discarded -= 1;
		}

		(self.player, self.assets, self.cards_discarded)
	}
}
