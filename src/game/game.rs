use crate::{
	deck::{Deck, DrawCount},
	game::{player::Player, player_q::PlayerQ, player_state::PlayerState},
};

use std::io::{stdin, stdout, Write};

#[derive(Debug)]
pub struct Game<'a> {
	table: Vec<PlayerState<'a>>,
	draw_pile: Deck,
	players: PlayerQ,
}

// TODO define player actions

impl Game<'_> {
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

			println!("Cards in your hand: {:#?}", player.hand);
			println!("Cards on the table: {:#?}", self.table);

			print!("What do you want to do? ");
			stdout().flush();
			stdin()
				.read_line(&mut user_input)
				.expect("Couldn't read from `stdin`... :<");

			println!("You entered {}.", user_input.trim());
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
