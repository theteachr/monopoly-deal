use crate::{cards::Colored, color::CardColor, common::input, deck::Deck, player::Player};

use std::collections::VecDeque;
use std::fmt::Debug;

struct Turn {
	player: Player,
	num_cards_played: u8,
}

enum PlayerAction {
	Play(u8),
	Pass,
	// TODO Rearrange
}

impl Turn {
	pub fn new(player: Player) -> Self {
		Self {
			player,
			num_cards_played: 0,
		}
	}

	pub fn read_action(&mut self) -> PlayerAction {
		if self.num_cards_played == 3 {
			return PlayerAction::Pass;
		}

		self.player.print_assets();
		self.player.hand.print_numbered();

		let user_input = input("> ");
		let trimmed = user_input.trim();

		if trimmed.is_empty() {
			return PlayerAction::Pass;
		}

		loop {
			match trimmed.parse::<u8>() {
				Ok(n) => break PlayerAction::Play(n),
				Err(_) => continue,
			}
		}
	}

	fn play(&mut self, card_position: u8, table: &mut VecDeque<Player>) {
		if let Some(n) = self
			.player
			.remove_card_at(card_position)
			.and_then(|card| card.play(table, &mut self.player))
		{
			println!("{}", n);
			self.num_cards_played += 1;
		}
	}

	fn terminate(mut self, table: &mut VecDeque<Player>, discard_pile: &mut Deck) {
		// A player is not allowed to have more than 7 cards in their hand at the end of a turn.
		// This needs to be checked at the end of each turn. The player should be propmted for discarding.
		let mut to_be_discarded: i8 = self.player.hand.len() as i8 - 7;

		println!("{} card(s) need to be discarded.", to_be_discarded);

		while to_be_discarded > 0 {
			self.player.hand.print_numbered();

			if let Some(card) = input("> ")
				.trim()
				.parse::<u8>()
				.ok()
				.and_then(|i| self.player.remove_card_at(i))
			{
				discard_pile.push_back(card);
                to_be_discarded -= 1;
			}
		}

		table.push_back(self.player);
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
		loop {
			match turn.read_action() {
				PlayerAction::Play(n) => turn.play(n, &mut self.players),
				PlayerAction::Pass => break,
			}
		}

		turn.terminate(&mut self.players, &mut self.discard_pile);
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
