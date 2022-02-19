use crate::{
	cards::{Card, Colored},
	color::CardColor,
	common::input,
	deck::Deck,
	player::Player,
};

use std::collections::{HashSet, VecDeque};
use std::{
	cmp::Ordering::{Equal, Greater},
	fmt::Debug,
};

struct Turn {
	player: Player,
	num_cards_played: u8,
}

impl Turn {
	pub fn new(player: Player) -> Self {
		Self {
			player,
			num_cards_played: 0,
		}
	}

	fn read_card(&mut self) -> Option<Card> {
		if self.num_cards_played == 3 {
			return None;
		}

		self.player.print_assets();
		self.player.print_numbered_hand();

		loop {
			match input("> ")
				.trim()
				.parse::<u8>()
				.ok()
				.and_then(|i| self.player.remove_card_at(i))
			{
				Some(card) => {
					self.num_cards_played += 1;
					break Some(card);
				}
				None => continue,
			}
		}
	}

	fn terminate(self) -> Player {
		self.player
	}
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
		while let Some(card) = turn.read_card() {
			card.play(&mut self.players, &mut turn.player);
		}

		let mut player = turn.terminate();

		self.handle_excess_cards(&mut player);
		self.players.push_back(player);
	}

	fn handle_excess_cards(&mut self, player: &mut Player) {
		// A player is not allowed to have more than 7 cards in their hand at the end of a turn.
		// This needs to be checked at the end of each turn. The player should be propmted for discarding.
		let card_count = player.hand.len();
		let to_be_discarded: i8 = card_count as i8 - 7;

		// Nothing to be discarded, return.
		if to_be_discarded <= 0 {
			return;
		}

		println!("You need to discard {}.", to_be_discarded);
		player.print_numbered_hand();

		let discard_numbers = loop {
			let nums = read_card_numbers(player);

			if nums.len() != to_be_discarded as usize {
				println!("You need to discard exactly {}.", to_be_discarded);
				continue;
			}

			break nums;
		};

		// Remove cards from player's hand and put them in the discard pile.
		for num in discard_numbers {
			self.discard_pile
				.push_back(player.hand.remove(num.into()).unwrap());
		}
	}

	fn print_table(&mut self) {
		for _ in 1..self.player_count {
			let player = self.players.pop_front().unwrap();

			player.print_assets();
			self.players.push_back(player);
		}
	}
}

fn read_card_numbers(player: &Player) -> Vec<u8> {
	let max_card_num = player.hand.len() - 1;

	loop {
		match input("> ")
			.trim()
			.split_whitespace()
			.map(|n| {
				n.parse::<u8>()
					.ok()
					.and_then(|x| match max_card_num.cmp(&x) {
						Greater | Equal => Some(x),
						_ => None,
					})
			})
			.collect::<Option<HashSet<u8>>>()
		{
			Some(nums) => break nums.into_iter().collect(),
			None => continue,
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
