use crate::cards::money_card::MoneyCard;
use crate::cards::property_card::PropertyCard;
use crate::deck::{Deck, DrawCount};
use crate::player::Player;

use std::collections::HashSet;

#[derive(Debug)]
struct PlayerState<'a> {
	properties: HashSet<&'a PropertyCard>,
	bank: HashSet<&'a MoneyCard>,
}

impl PlayerState<'_> {
	fn new() -> Self {
		Self { properties: HashSet::new(), bank: HashSet::new() }
	}
}

#[derive(Debug)]
struct PlayerQ {
	players: Vec<Player>,
	index: usize,
	size: usize,
}

impl PlayerQ {
	fn new(size: usize) -> Self {
		Self { index: 0, players: Vec::with_capacity(size as usize), size }
	}

	fn next(&mut self) -> &Player {
		let player = &self.players[self.index];
		self.index = (self.index + 1) % self.size;

		player
	}
}

impl From<Vec<Player>> for PlayerQ {
	fn from(players: Vec<Player>) -> Self {
		Self { index: 0, size: players.len(), players }
	}
}

#[derive(Debug)]
pub struct Game<'a> {
	table: Vec<PlayerState<'a>>,
	draw_pile: Deck,
	players: PlayerQ,
}

impl Game<'_> {
	pub fn new(num_players: u8) -> Self {
		let mut draw_pile = Deck::new();

		println!("Shuffled {} cards.", draw_pile.len());

		let mut players = get_mock_players();

		println!("Added {} players. {:#?}", players.len(), players);

		// distribute cards
		for player in &mut players {
			player.update_hand(draw_pile.draw(DrawCount::Five));
		}

		println!(
			"Gave 5 cards each. {} cards left in the deck.",
			draw_pile.len()
		);

		println!("{:#?}", players);

		Self {
			draw_pile,
			table: (0..num_players).map(|_| PlayerState::new()).collect(),
			players: PlayerQ::from(players),
		}
	}

	pub fn initiate(&mut self) {
		println!("The Deal has been initiated.");

		println!("{}'s turn.", self.players.next().name);
		println!("{}'s turn.", self.players.next().name);
		println!("{}'s turn.", self.players.next().name);
		println!("{}'s turn.", self.players.next().name);
		println!("{}'s turn.", self.players.next().name);
	}
}

fn get_mock_players() -> Vec<Player> {
	["Red", "Matilda", "Bomb", "Henry"]
		.iter()
			.enumerate()
			.map(|(i, name)| Player::new(i, String::from(*name)))
			.collect()
}
