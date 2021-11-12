use crate::game::player::Player;

#[derive(Debug)]
pub struct PlayerQ {
	players: Vec<Player>,
	index: usize,
	size: usize,
}

impl PlayerQ {
	pub fn new(size: usize) -> Self {
		Self { index: 0, players: Vec::with_capacity(size as usize), size }
	}

	pub fn next(&mut self) -> &mut Player {
		let player = &mut self.players[self.index];
		self.index = (self.index + 1) % self.size;

		player
	}
}

impl From<Vec<Player>> for PlayerQ {
	fn from(players: Vec<Player>) -> Self {
		Self { index: 0, size: players.len(), players }
	}
}
