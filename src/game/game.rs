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
			"Added {} players: {:?}.",
			players.len(),
			players
				.iter()
				.map(|p| p.name.as_str())
				.collect::<Vec<&str>>()
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

			let hand_cards = player.hand();
			let played_cards = player.played();

			println!(
				"{}. Your turn. You have {} card(s) in your hand.",
				player.name,
				hand_cards.len()
			);

			println!("Cards in your hand:");
			print_numbered_cards(&hand_cards);

			println!("Your cards:");
			print_numbered_cards(&played_cards);

			println!("Rest of the Table:");

			for _ in 1..self.player_count {
				let other_player = self.players.pop_front().unwrap();

				println!("{}'s Cards --->", other_player.name);
				print_numbered_cards(&other_player.played());
				self.players.push_back(other_player);
			}

			self.handle_player_actions(&mut player);
			self.players.push_back(player);
		}
	}

	fn handle_player_actions(&self, player: &mut Player) {
		while let Some(action) = read_action() {
			let text = match action {
				PlayerAction::Play => "play",
				PlayerAction::Pass => "pass",
				PlayerAction::Rearrange => "rearrange",
			};

			println!("You chose to {}.", text);
		}

		self.handle_player_actions(player);
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
	let mut input = String::new();

	for (i, action_text) in ACTION_TEXTS.iter().enumerate() {
		println!("{}: {}", i, action_text);
	}

	print!("What do you want to do? ");
	stdout().flush().expect("Couldn't flush :<");

	stdin()
		.read_line(&mut input)
		.expect("Couldn't read from `stdin`... :<");

	match input.trim().parse() {
		Ok(0) => Some(PlayerAction::Play),
		Ok(1) => Some(PlayerAction::Pass),
		Ok(2) => Some(PlayerAction::Rearrange),
		    _ => None,
	}
}
