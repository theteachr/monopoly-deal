use crate::{
	cards::card::Card,
	deck::{Deck, DrawCount},
	game::{player::Player, player_q::PlayerQ, player_state::PlayerState},
};

use std::io::{stdin, stdout, Write};

#[derive(Debug)]
pub struct Game {
	table: Vec<PlayerState>,
	draw_pile: Deck,
	players: PlayerQ,
}

// TODO define player actions

enum PlayerAction {
	PlayCard,
	Pass,
}

impl Game {
	pub fn new(_num_players: u8) -> Self {
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

		let table = players
			.iter()
			.map(|player| PlayerState::new(player.name.clone()))
			.collect();

		Self {
			draw_pile,
			table,
			players: PlayerQ::from(players),
		}
	}

	pub fn initiate(&mut self) {
		let mut user_input = String::new();

		println!("The Deal has been initiated.");

		// TODO take max three inputs

		loop {
			user_input.clear();

			let player = self.players.next();

			let cards_drawn = self.draw_pile.draw(DrawCount::Two);
			println!(
				"Drew {} cards from the deck. {} left.",
				cards_drawn.len(),
				self.draw_pile.len()
			);
			player.update_hand(cards_drawn);

			let cards_in_hand = player.cards_in_hand();

			println!(
				"{}. Your turn. You have {} card(s) in your hand.",
				player.name,
				cards_in_hand.len()
			);

			println!("Cards in your hand:");
			print_numbered_cards(&cards_in_hand);
			println!("Cards on the table:");

			print!("What do you want to do? ");
			stdout().flush();

			stdin()
				.read_line(&mut user_input)
				.expect("Couldn't read from `stdin`... :<");

			let card_num: usize = user_input.trim().parse().unwrap();
			let selected_card = cards_in_hand[card_num];

			println!("You chose {:?}.", selected_card);
		}
	}
}

fn get_mock_players() -> Vec<Player> {
	["Red", "Blue"]
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