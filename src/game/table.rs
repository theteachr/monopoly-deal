use crate::player::Assets;
use std::collections::VecDeque;

/// Represents the Game Table.
/// Holds all asset cards (property and money) played by the players.
#[derive(Debug)]
pub struct Table(VecDeque<Assets>);

impl Table {
	/// Gives an empty table with `player_count` slots.
	pub fn new(player_count: u8) -> Self {
		Self((0..player_count).map(|_| Assets::new()).collect())
	}

	/// Gives the assets of the next player and a reference to the rest of the table
	/// holding the assets of the other players.
	pub fn turn(&mut self) -> Assets {
		self.0.pop_front().unwrap()
	}

	/// Puts back the `assets` into the table.
	/// Called at the end of every turn.
	pub fn update(&mut self, assets: Assets) {
		self.0.push_back(assets);
	}

	pub fn print(&self) {
		self.0
			.iter()
			.enumerate()
			.for_each(|(i, assets)| println!("\n-- {} --\n\n{}", i, assets));
	}
}
