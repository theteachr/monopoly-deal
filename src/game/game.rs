use crate::{
	deck::{Deck, DrawCount},
	game::{player::Player, player_q::PlayerQ, player_state::PlayerState},
	cards::card::Card,
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
	pub fn new(num_players: u8) -> Self {
		let mut draw_pile = Deck::new();

		println!("Shuffled {} cards.", draw_pile.len());

		let mut players = get_mock_players();

		println!("Added {} players. {:#?}", players.len(), players);

		// distribute cards
		for player in &mut players {
			player.update_hand(draw_pile.draw(DrawCount::Five));
		}

		println!(
			"Gave 5 cards each. {} cards left in the deck.",
			draw_pile.len()
		);

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

			println!("{}. Your turn.", player.name);

			player.update_hand(self.draw_pile.draw(DrawCount::Two));

			let cards_in_hand = player.cards_in_hand();

			println!("Cards in your hand:");
			print_numbered_cards(&cards_in_hand);
			println!("Cards on the table:");

			let cards_played = self.table[player.id].cards();

			print_numbered_cards(&cards_played);

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
