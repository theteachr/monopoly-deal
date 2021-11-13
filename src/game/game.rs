use crate::{
	cards::{card::Card, card_set::CardSet},
	deck::{Deck, DrawCount},
	game::{player::Player, player_q::PlayerQ},
};

use std::io::{stdin, stdout, Write};

#[derive(Debug)]
pub struct Game {
	properties: Vec<CardSet>,
	monies: Vec<CardSet>,
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
			let cards_drawn = draw_pile.draw(DrawCount::Five);
			println!("Gave {:?} to {}.", &cards_drawn, player.name);
			player.update_hand(cards_drawn);
		}

		println!("{:#?}", players);

		Self {
			draw_pile,
			properties: CardSet::vec(num_players),
			monies: CardSet::vec(num_players),
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
			print_numbered_cards(&self.properties[player.id].cards());
			print_numbered_cards(&self.monies[player.id].cards());

			print!("Type the number of the card: ");
			stdout().flush();

			stdin()
				.read_line(&mut user_input)
				.expect("Couldn't read from `stdin`... :<");

			let card_position: usize = user_input.trim().parse().unwrap();
			let selected_card = player.hand.remove(card_position);

			println!("Adding {:?} to the table...", selected_card);

			let section = match selected_card {
				Card::Money(_) => &mut self.monies,
				Card::Property(_) => &mut self.properties,
				Card::Empty => unreachable!("Can't select an empty card."),
			};

			section[player.id].add(selected_card);
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
