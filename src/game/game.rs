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

		// TODO take max three inputs

		loop {
			let mut player = self.players.pop_front().unwrap();
			let cards_drawn = self.draw_pile.draw(DrawCount::Two);

			player.update_hand(cards_drawn);

			println!("{}. Your turn", player.name);

			println!("Hand: {}", cards_to_string(player.hand()));
			println!("Table: {}", cards_to_string(player.played()));

			self.table();

			self.handle_player_action(&mut player);
			self.players.push_back(player);
		}
	}

	fn handle_player_action(&mut self, player: &mut Player) {
		while let Some(action) = read_action() {
			match action {
				PlayerAction::Play      => self.handle_play(player),
				PlayerAction::Pass      => todo!(),
				PlayerAction::Rearrange => todo!(),
			}
		}

		println!("You can't do that :o");
		self.handle_player_action(player);
	}

	fn handle_play(&mut self, player: &mut Player) {
		println!("Cards in your hand:");
		print_numbered_cards(&player.hand());

		println!("Your cards:");
		print_numbered_cards(&player.played());

		println!("Rest of the Table:");

		let card_position: usize = input("Choose card: ").trim().parse().unwrap();
		let selected_card = player.hand.remove(card_position);

		player.played.add(selected_card);
	}

	fn table(&mut self) {
		for _ in 1..self.player_count {
			let player = self.players.pop_front().unwrap();

			println!(
				"{}'s Cards: {}",
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

fn read_action() -> Option<PlayerAction> {
	for (i, action_text) in ACTION_TEXTS.iter().enumerate() {
		println!("{}: {}", i, action_text);
	}

	match input("What do you want to do? ").trim().parse() {
		Ok(0) => Some(PlayerAction::Play),
		Ok(1) => Some(PlayerAction::Pass),
		Ok(2) => Some(PlayerAction::Rearrange),
		_ => None,
	}
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
