use super::{Table, Turn};
use crate::{
	cards::{CardKind, CardSet, Colored},
	color::CardColor,
	common::input,
	deck::Deck,
	player::{Assets, Player},
};

use std::collections::VecDeque;
use std::fmt::Debug;

pub enum PlayerAction {
	Play(usize),
	Pass,
}

#[derive(Debug)]
pub struct Game {
	draw_pile: Deck,
	discard_pile: Deck,
	players: VecDeque<Player>,
	table: Table,
}

impl Game {
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

	pub fn initiate(&mut self) {
		println!("The Deal has been initiated.");

		loop {
			let mut player = self.players.pop_front().unwrap();
			let (assets, table) = self.table.turn();

			player.draw(&mut self.draw_pile);

			let (player, assets, discarded) = handle_turn(Turn::new(player, assets), table);

			discarded
				.into_iter()
				.for_each(|card| self.discard_pile.push_back(card));

			self.players.push_back(player);
			self.table.update(assets);
		}
	}
}

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
		if let Ok(n) = input("Choose color: ").parse::<u8>() {
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
