use crate::player::{Assets, Player};
use std::collections::VecDeque;
use std::fmt;

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
		(
			self.players.pop_front().unwrap(),
			self.assets.pop_front().unwrap(),
		)
	}

	/// Puts back the `assets` into the table.
	/// Called at the end of every turn.
	pub fn update(&mut self, player: Player, assets: Assets) {
		self.players.push_back(player);
		self.assets.push_back(assets);
	}

	pub fn get_mut_assets(&mut self, idx: usize) -> Option<&mut Assets> {
		self.assets.get_mut(idx)
	}

	pub fn iter_mut(&mut self) -> impl Iterator<Item = (&Player, &mut Assets)> {
		self.players.iter().zip(self.assets.iter_mut())
	}
}

impl fmt::Display for Table {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.assets
			.iter()
			.zip(self.players.iter())
			.enumerate()
			.map(|(i, (assets, player))| format!("\n{}: -- {} --\n\n{}", i, player.name, assets))
			.collect::<String>()
			.fmt(f)
	}
}
