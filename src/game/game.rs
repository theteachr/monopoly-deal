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

use PlayerInputState::*;

impl Game {
	pub fn new(player_count: u8) -> Self {
		let mut draw_pile = Deck::new();

		println!("Shuffled {} cards.", draw_pile.len());

		let mut players = get_mock_players();

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

			self.handle_player_action(&mut player);
			self.handle_excess_cards(&mut player);

			self.players.push_back(player);
		}
	}

	fn handle_player_action(&mut self, player: &mut Player) {
		let mut count = 0;

		while let Ok(state) = read_action(&mut count) {
			println!(
				"{}'s played cards: {}",
				player.name,
				cards_to_string(player.played())
			);

			match state {
				Continue(action) => match action {
					PlayerAction::Play      => self.handle_play(player),
					PlayerAction::Pass      => todo!(), // should not be reachable
					PlayerAction::Rearrange => todo!(),
				},
				Stop => return,
			}
		}

		// TODO Handle excess cards in hand (<= 7)
		// TODO Handle wrong card selection

		println!("{}, you can't do that :o", player.name);
		self.handle_player_action(player);
	}

	fn handle_play(&mut self, player: &mut Player) {
		print_numbered_cards(&player.hand());

		let card_position: usize = input("Choose card: ").trim().parse().unwrap();

		player.play_card_at(card_position);
	}

	fn handle_excess_cards(&self, player: &mut Player) {
		// A player is not allowed to have more than 7 cards in their hand at theend of a turn.
		// This needs to be checked at the end of each turn. The player should be propmted for discarding.
		let card_count = player.hand.len();

		if card_count > 7 {
			println!("You need to discard {}.", card_count - 7);
		}
	}

	fn table(&mut self) {
		for _ in 1..self.player_count {
			let player = self.players.pop_front().unwrap();

			println!(
				"{}'s cards: {}",
				player.name,
				cards_to_string(player.played())
			);

			self.players.push_back(player);
		}
	}
}

fn get_mock_players() -> Vec<Player> {
	["Rick", "Morty"]
		.iter()
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
		return Ok(PlayerInputState::Stop);
	}

	for (i, action_text) in ACTION_TEXTS.iter().enumerate() {
		println!("{}: {}", i, action_text);
	}

	let (action, update) = match input("What do you want to do? ").trim().parse() {
		Ok(0) => (PlayerAction::Play, *count + 1),
		Ok(1) => (PlayerAction::Pass, *count),
		Ok(2) => (PlayerAction::Rearrange, *count),
		_ => return Err("You can't do that :o"),
	};

	if let PlayerAction::Pass = action {
		return Ok(PlayerInputState::Stop);
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
		.expect("Couldn't read from `stdin`... :<");

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
