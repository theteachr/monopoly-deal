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
	num_players: u8,
}

// TODO define player actions

impl Game {
	pub fn new(num_players: u8) -> Self {
		let mut draw_pile = Deck::new();

		println!("Shuffled {} cards.", draw_pile.len());

		let mut players = get_mock_players();

		println!("Added {} players. {:#?}", players.len(), players);

		// distribute cards
		for player in &mut players {
			let cards_drawn = draw_pile.draw(DrawCount::Five);
			println!("Gave {:?} to {}.", &cards_drawn, player.name);
			player.update_hand(cards_drawn);
		}

		println!("{:#?}", players);

		Self {
			draw_pile,
			players: VecDeque::from(players),
			num_players,
		}
	}

	pub fn initiate(&mut self) {
		let mut user_input = String::new();

		println!("The Deal has been initiated.");

		// TODO take max three inputs

		loop {
			user_input.clear();

			let mut player = self.players.pop_front().unwrap();

			let cards_drawn = self.draw_pile.draw(DrawCount::Two);
			println!(
				"Drew {} cards from the deck. {} left.",
				cards_drawn.len(),
				self.draw_pile.len()
			);

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

			for _ in 1..self.num_players {
				let other_player = self.players.pop_front().unwrap();

				println!("{}'s Cards --->", other_player.name);
				print_numbered_cards(&other_player.played());
				self.players.push_back(other_player);
			}

			print!("Type the number of the card: ");
			stdout().flush().expect("Couldn't flush :<");

			stdin()
				.read_line(&mut user_input)
				.expect("Couldn't read from `stdin`... :<");

			let card_position: usize = user_input.trim().parse().unwrap();
			let selected_card = player.hand.remove(card_position);

			println!("Adding {:?} to the table...", selected_card);

			player.played.add(selected_card);
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
		println!("{}: {:?}", i, card);
	}
}
