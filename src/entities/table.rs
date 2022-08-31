use super::{Assets, Player};
use std::collections::VecDeque;
use std::fmt;

/// Represents the Game Table.
///
/// Holds two parallel queues of players and their respective assets.
#[derive(Debug)]
pub struct Table {
	/// A queue of players.
	players: VecDeque<Player>,

	/// A queue of assets played by the respective players.
	assets: VecDeque<Assets>,
}

impl Table {
	/// Returns a table with the supplied `players` with a corresponding
	/// empty slot in the `assets` queue for each.
	pub fn new(players: Vec<Player>) -> Self {
		Self {
			assets: players.iter().map(|_| Assets::new()).collect(),
			players: VecDeque::from(players),
		}
	}

	/// Returns the tuple of the next player and their assets.
	pub fn turn(&mut self) -> (Player, Assets) {
		(
			self.players.pop_front().unwrap(),
			self.assets.pop_front().unwrap(),
		)
	}

	/// Puts back the `player` and their updated `assets` into the table.
	pub fn update(&mut self, player: Player, assets: Assets) {
		self.players.push_back(player);
		self.assets.push_back(assets);
	}

	/// Returns a mutable ref to the assets at `idx` of the assets queue.
	pub fn get_mut_assets(&mut self, idx: usize) -> Option<&mut Assets> {
		self.assets.get_mut(idx)
	}

	/// Returns an iterator of players zipped with the mutable ref to their assets.
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
