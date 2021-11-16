use crate::{
	cards::card::Card,
	deck::{Deck, DrawCount},
	game::player::Player,
};

use std::collections::VecDeque;
use std::io::{stdin, stdout, Write};

#[derive(Debug)]
pub struct Game {
	draw_pile: Deck,
	discard_pile: Deck,
	players: VecDeque<Player>,
	player_count: u8,
}

const ACTION_TEXTS: [&str; 3] = ["Play a card", "Pass", "Rearrange"];

#[repr(u8)]
enum PlayerAction {
	Play,
	Pass,
	Rearrange,
}

#[repr(u8)]
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

			self.handle_player_action(&mut player);
			self.handle_excess_cards(&mut player);

			self.players.push_back(player);
		}
	}

	fn handle_player_action(&mut self, player: &mut Player) {
		let mut count = 0;

		while let Ok(state) = read_action(&mut count) {
			match state {
				Continue(action) => match action {
					Play => self.handle_play(player),
					Pass => todo!(), // should not be reachable
					Rearrange => todo!(),
				},
				Stop => return,
			}
		}

		println!("{}, you can't do that :o", player.name);
		self.handle_player_action(player);
	}

	fn handle_play(&mut self, player: &mut Player) {
		print_numbered_cards(&player.hand());

		let card_position: usize = choose_card(player.hand.len());

		// XXX read numbers of all cards the user wants to play (will require the numbers be
		// sorted in descending order)

		player.play_card_at(card_position);
		println!(
			"{}'s assets: {}",
			player.name,
			cards_to_string(player.played())
		);
		println!("{}'s hand: {}", player.name, cards_to_string(player.hand()));
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
			let card = player.hand.remove(choose_card(player.hand.len()));
			self.discard_pile.add(card);
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

fn read_action(count: &mut u8) -> Result<PlayerInputState, &str> {
	if *count == 3 {
		return Ok(Stop);
	}

	for (i, action_text) in ACTION_TEXTS.iter().enumerate() {
		println!("{}: {}", i, action_text);
	}

	let (action, update) = match input("What do you want to do? ").trim().parse() {
		Ok(0) => (Play, *count + 1),
		Ok(1) => (Pass, *count),
		Ok(2) => (Rearrange, *count),
		_ => return Err("You can't do that :o"),
	};

	if let Pass = action {
		return Ok(Stop);
	}

	*count = update;

	return Ok(Continue(action));
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

fn choose_card(card_count: usize) -> usize {
	if let Ok(n) = input("Choose card: ").trim().parse() {
		if n < card_count {
			return n;
		}
	}

	println!(
		"That can't be chosen, please enter a number between 0 and {}.",
		card_count - 1
	);

	return choose_card(card_count);
}
