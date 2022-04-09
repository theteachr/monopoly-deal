use crate::player::{Assets, Player};
use std::collections::VecDeque;

/// Represents the Game Table.
/// Holds all asset cards (property and money) played by the players.
#[derive(Debug)]
pub struct Table {
	players: VecDeque<Player>,
	assets: VecDeque<Assets>,
}

impl Table {
	/// Gives an empty table with `player_count` slots.
	pub fn new(players: Vec<Player>) -> Self {
		Self {
			assets: players.iter().map(|_| Assets::new()).collect(),
			players: VecDeque::from(players),
		}
	}

	/// Gives the assets of the next player and a reference to the rest of the table
	/// holding the assets of the other players.
	pub fn turn(&mut self) -> (Player, Assets) {
		(self.players.pop_front().unwrap(), self.assets.pop_front().unwrap())
	}

	/// Puts back the `assets` into the table.
	/// Called at the end of every turn.
	pub fn update(&mut self, player: Player, assets: Assets) {
		self.players.push_back(player);
		self.assets.push_back(assets);
	}

	pub fn assets_iter_mut(&mut self) -> impl Iterator<Item = &mut Assets> {
		self.assets.iter_mut()
	}

	pub fn print(&self) {
		self.assets
			.iter()
			.enumerate()
			.for_each(|(i, assets)| println!("\n-- {} --\n\n{}", i, assets));
	}
}
