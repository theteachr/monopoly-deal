use crate::{
	cards::card::Card,
	deck::{Deck, DrawCount},
	game::player::Player,
};

use std::collections::{HashSet, VecDeque};
use std::io::{stdin, stdout, Write};

#[derive(Debug)]
pub struct Game {
	draw_pile: Deck,
	discard_pile: Deck,
	players: VecDeque<Player>,
	player_count: u8,
}

const ACTION_TEXTS: [&str; 3] = ["Play a card", "Pass", "Rearrange"];

enum PlayerAction {
	Play,
	Pass,
	Rearrange,
}

enum PlayerInputState {
	Continue(PlayerAction),
	Stop,
}

use PlayerAction::*;
use PlayerInputState::*;

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
			let cards_drawn = draw_pile.draw(DrawCount::Five);
			player.update_hand(cards_drawn);
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
			let draw_count = if player.hand.len() == 0 {
				DrawCount::Five
			} else {
				DrawCount::Two
			};
			let cards_drawn = self.draw_pile.draw(draw_count);

			player.update_hand(cards_drawn);

			println!("{}'s turn.", player.name);

			self.table();
			println!(
				"{}'s assets: {}",
				player.name,
				cards_to_string(player.played())
			);
			println!("{}'s hand: {}", player.name, cards_to_string(player.hand()));

			// TODO: Use a struct to maintain the states needed for a turn

			self.handle_player_action(&mut player);
			self.handle_excess_cards(&mut player);

			self.players.push_back(player);
		}
	}

	fn handle_player_action(&mut self, player: &mut Player) {
		let mut cards_played = 0;

		while let Some(action) = read_action(&mut cards_played) {
			match action {
				Play => self.handle_play(player),
				Pass => return,
				Rearrange => todo!(),
			};
		}
	}

	fn handle_play(&mut self, player: &mut Player) {
		let card_positions = read_card_numbers(&player.hand());

		println!("{:?}", card_positions);
		self.play_cards(player, card_positions);

		println!(
			"{}'s assets: {}",
			player.name,
			cards_to_string(player.played())
		);
		println!("{}'s hand: {}", player.name, cards_to_string(player.hand()));
	}

	fn play_cards(&self, player: &mut Player, card_positions: Vec<u8>) {
		for pos in card_positions.into_iter() {
			player.play_card_at(pos as usize);
		}
	}

	fn handle_excess_cards(&mut self, player: &mut Player) {
		// A player is not allowed to have more than 7 cards in their hand at the end of a turn.
		// This needs to be checked at the end of each turn. The player should be propmted for discarding.
		let card_count = player.hand.len();
		let to_be_discarded: i8 = card_count as i8 - 7;

		if to_be_discarded <= 0 {
			return;
		}

		println!("You need to discard {}.", to_be_discarded);

		for _ in 0..to_be_discarded {
			print_numbered_cards(&player.hand());
			// let card = player.hand.remove(read_card_numbers(player.hand.len()));
			// self.discard_pile.add(card);
		}
	}

	fn table(&mut self) {
		for _ in 1..self.player_count {
			let player = self.players.pop_front().unwrap();

			println!(
				"{}'s assets: {}",
				player.name,
				cards_to_string(player.played())
			);

			self.players.push_back(player);
		}
	}
}

fn get_mock_players(count: u8) -> Vec<Player> {
	["Rick", "Morty", "Pupa", "Gourd", "Harge"]
		.iter()
		.take(count as usize)
		.enumerate()
		.map(|(i, name)| Player::new(i, String::from(*name)))
		.collect()
}

fn print_numbered_cards(cards: &Vec<&Card>) {
	for (i, card) in cards.iter().enumerate() {
		println!("{}: {}", i, card);
	}
}

fn read_action(cards_played: &mut u8) -> Option<PlayerAction> {
	if *cards_played > 3 {
		return None;
	}

	let prompt = format!("({}) What do you want to do? ", *cards_played + 1);

	for (i, action_text) in ACTION_TEXTS.iter().enumerate() {
		println!("{}: {}", i, action_text);
	}

	let (action, update) = loop {
		match input(&prompt).trim().parse() {
			Ok(0) => break (Play, *cards_played + 1),
			Ok(1) => break (Pass, *cards_played),
			Ok(2) => break (Rearrange, *cards_played),
			_ => continue,
		};
	};

	*cards_played = update;

	return Some(action);
}

fn input(prompt: &str) -> String {
	let mut input = String::new();

	print!("{}", prompt);
	stdout().flush().expect("Couldn't flush :<");

	stdin()
		.read_line(&mut input)
		.expect("Couldn't read from `stdin` :<");

	return input;
}

fn cards_to_string(cards: Vec<&Card>) -> String {
	format!(
		"[{}]",
		cards
			.iter()
			.map(|card| card.to_string())
			.collect::<Vec<String>>()
			.join("; ")
	)
}

fn read_card_numbers(cards: &Vec<&Card>) -> Vec<u8> {
	print_numbered_cards(cards);

	let mut results = loop {
		match input("Enter the numbers of cards that you want to play: ")
			.trim()
			.split(" ")
			.map(|n| n.parse().ok())
			.collect::<Option<HashSet<u8>>>()
		{
			Some(nums) => {
				if nums.iter().all(|&n| (n as usize) < cards.len()) {
					break nums.into_iter().collect::<Vec<u8>>();
				}
			}
			None => continue,
		};
	};

	results.sort_by_key(|k| std::cmp::Reverse(*k));

	return results;
}
