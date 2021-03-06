use super::{CurrentPlayer, PlayerAction, Table};
use crate::{
	cards::{CardKind, CardSet},
	deck::Deck,
	player::{Assets, Player},
};

use std::fmt::Debug;

/// Represents the main game object.
#[derive(Debug)]
pub struct Game {
	/// Represents the deck of cards.
	pub deck: Deck,

	/// Used to hold action cards and those the players choose to discard when excess.
	pub discard_deck: Deck,

	/// Holds all players with the assets they play.
	pub table: Table,
}

impl Game {
	/// Returns a `Game` with 4 hard coded players and a shuffled deck of 108 cards.
	pub fn new(player_count: u8) -> Self {
		let mut draw_pile = Deck::new();
		let discard_pile = Deck::new();

		println!("Shuffled {} cards.", draw_pile.len());

		let mut players = get_mock_players(player_count);

		// Distribute cards. Since every player's hand be empty at start, each player
		// will be drawing *5* cards.
		for player in &mut players {
			player.draw(&mut draw_pile);
		}

		Self {
			deck: draw_pile,
			discard_deck: discard_pile,
			table: Table::new(players),
		}
	}

	/// Starts the game loop.
	pub fn play(&mut self) {
		// Get the next player and their assets.
		let (mut player, assets) = self.table.turn();

		// Make the player draw cards from the deck.
		player.draw(&mut self.deck);

		// Get the updated player and their assets, along with the set of cards
		// that they chose to discard.
		let (player, assets, discarded) = self.handle_turn(CurrentPlayer::new(player, assets));

		// Put the discarded ones into the discard deck.
		discarded
			.into_iter()
			.for_each(|card| self.discard_deck.push_back(card));

		// Put the player back into the queue.
		self.table.update(player, assets);
	}

	/// Returns the updated player, their assets and the set of cards they chose to discard.
	fn handle_turn(&mut self, mut player: CurrentPlayer) -> (Player, Assets, CardSet<CardKind>) {
		loop {
			println!("{}", self.table);

			match player.read_action() {
				PlayerAction::Play(card) => card.play(&mut player, self),
				PlayerAction::Pass => break,
				PlayerAction::Rearrange => unimplemented!(),
			}
		}

		player.end_turn()
	}
}

/// Returns a vector of `count` players.
fn get_mock_players(count: u8) -> Vec<Player> {
	["Rick", "Morty", "Pupa", "Gourd", "Harge"]
		.iter()
		.take(count as usize)
		.enumerate()
		.map(|(i, &name)| Player::new(i, String::from(name)))
		.collect()
}
