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
			let cards_drawn = self.draw_pile.draw(DrawCount::Two);

			player.update_hand(cards_drawn);

			println!("{}'s turn.", player.name);

			self.table();

			self.handle_player_action(&mut player);
			self.players.push_back(player);
		}
	}

	fn handle_player_action(&mut self, player: &mut Player) {
		let mut count = 0;

		while let Ok(state) = read_action(&mut count) {
			println!("{}'s hand: {}", player.name, cards_to_string(player.hand()));
			println!("{}'s played cards: {}", player.name, cards_to_string(player.played()));

			match state {
				PlayerInputState::Continue(action) => match action {
					PlayerAction::Play      => self.handle_play(player),
					PlayerAction::Pass      => todo!(), // should not be reachable
					PlayerAction::Rearrange => todo!(),
				},
				PlayerInputState::Stop => return,
			}
		}

		// TODO Handle excess cards in hand (<= 7)

		println!("{}, you can't do that :o", player.name);
		self.handle_player_action(player);
	}

	fn handle_play(&mut self, player: &mut Player) {
		println!("{}'s Hand:", player.name);
		print_numbered_cards(&player.hand());

		let card_position: usize = input("Choose card: ").trim().parse().unwrap();

		player.play_card_at(card_position);
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

	return Ok(PlayerInputState::Continue(action));
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
