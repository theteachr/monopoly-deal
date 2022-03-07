use super::PlayerAction;
use crate::cards::{Card, CardKind, CardSet};
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
		print_indexed(self.player.hand.iter());

		println!();

		// Print total money in the bank.
		println!("Total Bank Value: {}", self.assets.bank_value());

		// Print total value of all the properties.
		println!("Total Asset Value: {}", self.assets.total_property_value());

		println!();

		loop {
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
				.and_then(|n| self.player.hand.get(n).ok_or(Failed::InvalidIndex))
				.and_then(|card| card.is_playable(&self.assets).map_err(Failed::from))
			{
				Ok(_) => break PlayerAction::Play(self.player.remove_card_at(parsed.unwrap())),
				Err(e) => println!("{}", e),
			}
		}
	}

	/// Returns the mutated player, assets and a set of cards the player chose to discard.
	///
	/// A player is not allowed to have more than **7** cards in their hand at the end of a turn.
	/// This needs to be checked at the end of each turn. If there are excess cards, prompt
	/// the player to discard the excess.
	pub fn end_turn(mut self) -> (Player, Assets, CardSet<CardKind>) {
		// Get the number of cards that need to be discarded.
		let mut to_be_discarded: i8 = self.player.hand.len() as i8 - 7;

		// Until the number of cards need to be discarded is > 0,
		// ask the player to enter the index of the card they want to discard.
		while to_be_discarded > 0 {
			println!("You need to discard {}.", to_be_discarded);

			println!();

			// Print the cards in hand along with their index.
			print_indexed(self.player.hand.iter());

			println!();

			// Read the index from the user.
			let card_position = read_index("> ", self.player.hand.len());

			// Remove the card at `card_position` from the  player's hand,
			// and add it to the set of cards that will be dumped in the disccard pile.
			self.cards_discarded
				.add(self.player.remove_card_at(card_position));

			// One less card to be discarded, decrement the variable.
			to_be_discarded -= 1;
		}

		(self.player, self.assets, self.cards_discarded)
	}
}
