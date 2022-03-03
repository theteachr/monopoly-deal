use crate::player::Assets;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Table(VecDeque<Assets>);

impl Table {
	pub fn new(player_count: u8) -> Self {
		Self((0..player_count).map(|_| Assets::new()).collect())
	}

	pub fn turn(&mut self) -> (Assets, &mut Self) {
		let assets = self.0.pop_front().unwrap();

		(assets, self)
	}

	pub fn update(&mut self, assets: Assets) {
		self.0.push_back(assets);
	}

	pub fn print(&self) {
		self.0.iter().enumerate().for_each(|(i, assets)| println!("\n-- {} --\n\n{}", i, assets));
	}
}
