use super::Turn;
use crate::{cards::Colored, color::CardColor, common::input, deck::Deck, player::Player};

use std::collections::VecDeque;
use std::fmt::Debug;

pub(crate) enum PlayerAction {
	Play(u8),
	Pass,
	// TODO Rearrange
}

#[derive(Debug)]
pub struct Game {
	draw_pile: Deck,
	discard_pile: Deck,
	players: VecDeque<Player>,
	player_count: u8,
}

impl Game {
	pub fn new(player_count: u8) -> Self {
		let mut draw_pile = Deck::new();

		println!("Shuffled {} cards.", draw_pile.len());

		let mut players = get_mock_players(player_count);

		println!(
			"Added {} players: {}.",
			players.len(),
			players
				.iter()
				.map(|p| p.name.as_str())
				.collect::<Vec<&str>>()
				.join(", ")
		);

		// distribute cards
		for player in &mut players {
			player.draw(&mut draw_pile);
		}

		Self {
			draw_pile,
			discard_pile: Deck::new(),
			players: VecDeque::from(players),
			player_count,
		}
	}

	pub fn initiate(&mut self) {
		println!("The Deal has been initiated.");

		loop {
			let mut player = self.players.pop_front().unwrap();

			player.draw(&mut self.draw_pile);
			println!("{}'s turn.", player.name);
			self.print_table();

			self.handle_turn(Turn::new(player));
		}
	}

	fn handle_turn(&mut self, mut turn: Turn) {
		loop {
			match turn.read_action() {
				PlayerAction::Play(n) => turn.play(n, &mut self.players),
				PlayerAction::Pass => break,
			}
		}

		let (mut player, cards_played, cards_discarded) = turn.terminate();

		cards_played.into_iter().for_each(|card| card.play(&mut self.players, &mut player));

		self.players.push_back(player);
	}

	fn print_table(&mut self) {
		for _ in 1..self.player_count {
			let player = self.players.pop_front().unwrap();

			player.print_assets();
			self.players.push_back(player);
		}
	}
}

fn get_mock_players(count: u8) -> Vec<Player> {
	["Rick", "Morty", "Pupa", "Gourd", "Harge"]
		.iter()
		.take(count as usize)
		.enumerate()
		.map(|(i, &name)| Player::new(i, String::from(name)))
		.collect()
}

pub fn read_color<T: Colored>(card: &T) -> CardColor {
	let colors = card.colors();
	let max_choose_num = colors.len();

	for (i, color) in colors.iter().enumerate() {
		println!("{}: {}", i, color);
	}

	// FIXME: Smell -> repeating pattern of looping until right input
	loop {
		if let Ok(n) = input("Choose color: ").trim().parse::<u8>() {
			if (n as usize) < max_choose_num {
				break colors[n as usize];
			}
		}

		println!(
			"Invalid color number, entered value should be between 0..={}.",
			max_choose_num
		);
	}
}
