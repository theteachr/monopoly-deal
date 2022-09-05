use super::{Assets, Player};
use crate::cards::{Action, Card, CardKind};
use crate::common::{input, print_enumerated};
use crate::errors::Failed;

/// Represents the actions a player can perform in their turn.
pub enum PlayerAction {
	/// Holds the the card that the player chose to play.
	Play(usize),

	/// FIXME
	#[allow(dead_code)]
	Rearrange,

	/// Marks the end of the turn.
	Pass,
}

/// Stores all necessary information about the player playing the current turn.
pub struct CurrentPlayer {
	/// Represents the player playing the turn.
	pub player: Player,

	/// Represents the cards played by the player playing the turn.
	pub assets: Assets,

	/// Represents the number of cards played by the player in the turn.
	pub num_cards_played: u8,
}

impl CurrentPlayer {
	pub fn new(player: Player, assets: Assets) -> Self {
		Self {
			player,
			assets,
			num_cards_played: 0,
		}
	}

	/// Returns a mutable reference to the player playing the turn.
	pub fn get(&mut self) -> &mut Player {
		&mut self.player
	}

	/// Returns the action the player chooses to play.
	pub fn read_action(&self) -> PlayerAction {
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

		// Show total money in the bank.
		println!("Total Bank Value: {}", self.assets.bank_value());

		// Show total value of all the properties.
		println!("Total Asset Value: {}", self.assets.total_property_value());

		loop {
			// Show the player's hand.
			// Read the input from the user.
			let user_input = input("> ");

			// Let the player signal a `Pass` by just pressing the enter or the return key.
			match user_input.as_str() {
				s if s.is_empty() => break PlayerAction::Pass,
				"p" => break PlayerAction::Play(self.player.choose_card_from_hand()),
				_ => eprintln!("Unknown command"),
			}
		}
	}

	fn can_play<'b>(&self, card: &'b CardKind) -> Result<&'b CardKind, Failed> {
		// `DoubleTheRent` needs to be handled separately as it's the only card
		// that needs to know the cards the player has played in the turn to
		// determine if it's playable.
		if let CardKind::Action(card) = card {
			if matches!(card.action, Action::DoubleTheRent) {
				// If the player has played a rent card, `DoubleTheRent` card
				// is playable.
				return Err(Failed::NotPlayable(
					"DoubleTheRent is tricky...".to_string(),
				));
			}
		}

		match card
			.is_playable(&self.assets.property_sets)
			.map_err(Failed::from)
		{
			Ok(()) => Ok(card),
			Err(e) => Err(e),
		}
	}

	/// A player is not allowed to have more than **7** cards in their hand at the end of a turn.
	/// This needs to be checked at the end of each turn. If there are excess cards, prompt
	/// the player to discard them.
	pub fn end_turn(mut self) -> (Player, Assets, Vec<CardKind>) {
		// Get the number of cards that need to be discarded.
		let mut to_be_discarded = self.player.num_cards_in_hand().saturating_sub(7);
		let mut discarded = Vec::new();

		// Until the number of cards need to be discarded is > 0,
		// ask the player to enter the index of the card they want to discard,
		// and add the card into the discard pile.
		while to_be_discarded > 0 {
			// Read and remove card from hand.
			println!("You need to discard {}.", to_be_discarded);

			let card_id = self.player.choose_card_from_hand();
			let card = self.player.take_from_hand(card_id).unwrap();

			discarded.push(card);
			// One less card to be discarded, decrement the variable.
			to_be_discarded -= 1;
		}

		(self.player, self.assets, discarded)
	}
}
