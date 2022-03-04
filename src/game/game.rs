use super::{Table, Turn};
use crate::{
	cards::{CardKind, CardSet, Colored},
	color::CardColor,
	common::read_index,
	deck::Deck,
	player::{Assets, Player},
};

use std::collections::VecDeque;
use std::fmt::Debug;

/// Represents the actions a player can perform in their turn.
pub enum PlayerAction {
	/// Holds the index of the card that the player chose to play.
	Play(usize),

	/// Marks the end of the turn.
	Pass,
}

/// Represents the main game object.
#[derive(Debug)]
pub struct Game {
	/// Represents the deck of cards.
	draw_pile: Deck,

	/// Used to hold action cards and those the players choose to discard when excess.
	discard_pile: Deck,

	/// A queue of players in the game.
	players: VecDeque<Player>,

	/// Holds all cards played by every player.
	table: Table,
}

impl Game {
	/// XXX Returns a `Game` with 4 hard coded players and a shuffled deck of 108 cards.
	pub fn new(player_count: u8) -> Self {
		let mut draw_pile = Deck::new();
		let discard_pile = Deck::new();

		println!("Shuffled {} cards.", draw_pile.len());

		let mut players = get_mock_players(player_count);

		// distribute cards
		for player in &mut players {
			player.draw(&mut draw_pile);
		}

		Self {
			draw_pile,
			discard_pile,
			players: VecDeque::from(players),
			table: Table::new(player_count),
		}
	}

	/// Starts the game loop.
	pub fn initiate(&mut self) {
		println!("The Deal has been initiated.");

		loop {
			// Get the next player.
			let mut player = self.players.pop_front().unwrap();

			// Get the next player's assets and a mutable reference
			// to the rest of the table holding other players' played cards.
			let (assets, table) = self.table.turn();

			// Make the player draw cards from the deck.
			player.draw(&mut self.draw_pile);

			// Get the updated player and their assets, along with the set of cards
			// that they chose to discard.
			let (player, assets, discarded) = handle_turn(Turn::new(player, assets), table);

			// Put the discarded into the discard deck.
			discarded
				.into_iter()
				.for_each(|card| self.discard_pile.push_back(card));

			// Put the player back into the queue.
			self.players.push_back(player);

			// Add the updated player assets back onto the table.
			self.table.update(assets);
		}
	}
}

/// Returns updated player, their assets and a set of cards they chose to discard.
///
/// # Arguments
/// * `turn` - a `Turn` holding the player playing the turn and their assets
/// * `table` - a mutable ref to a `Table` holding cards played by rest of the players
fn handle_turn(mut turn: Turn, table: &mut Table) -> (Player, Assets, CardSet<CardKind>) {
	loop {
		table.print();

		match turn.read_action() {
			PlayerAction::Play(n) => turn.play(n),
			PlayerAction::Pass => break,
		}
	}

	turn.terminate()
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

/// Returns the color of the card that the player chose to play.
pub fn read_color<T: Colored>(card: &T) -> CardColor {
	// get all colors available on the card
	let colors = card.colors();

	// read the index from the user and return the color at the index
	colors[read_index("Choose color: ", colors.iter(), colors.len())]
}
