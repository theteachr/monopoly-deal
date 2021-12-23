use crate::{
	cards::Card,
	deck::{Deck, DrawCount},
	game::player::{Player, PlayerAction::*},
};

use std::collections::VecDeque;

#[derive(Debug)]
pub struct Game {
	draw_pile: Deck,
	discard_pile: Deck,
	players: VecDeque<Player>,
	player_count: u8,
}

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

			let draw_count = match player.hand.len() {
				0 => DrawCount::Five,
				_ => DrawCount::Two,
			};

			let cards_drawn = self.draw_pile.draw(draw_count);

			player.update_hand(cards_drawn);

			println!("{}'s turn.", player.name);
			self.table();

			// TODO: Use a struct to maintain the states needed for a turn?

			self.handle_player_action(&mut player);
			self.handle_excess_cards(&mut player);

			self.players.push_back(player);
		}
	}

	fn handle_player_action(&mut self, player: &mut Player) {
		player.print_numbered_hand();

		let mut card_positions = loop {
			let mut card_positions = Vec::new();
			let actions = player.read_actions();

			for action in actions {
				match action {
					Play(n) => card_positions.push(n),
					Rearrange => todo!(),
				}
			}

			if card_positions.len() > 3 {
				println!("You can't play more than 3 cards on a turn.");
				continue;
			}

			break card_positions;
		};

		// FIXME Sorting is necessary, but this place doesn't feel right.
		card_positions.sort_by_key(|k| std::cmp::Reverse(*k));

		for pos in card_positions {
			let selected_card = player.hand.remove(pos.into());

			match selected_card {
				Card::Property(_) | Card::Money(_) => player.played.add(selected_card),
				Card::Action(a) => println!("Action Card impl to be done: {}", a),
				Card::Wild(w) => println!("Wild Card impl to be done: {}", w),
			};
		}
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
			player.print_hand();
			// let card = player.hand.remove(read_card_numbers(player.hand.len()));
			// self.discard_pile.add(card);
		}
	}

	fn table(&mut self) {
		for _ in 1..self.player_count {
			let player = self.players.pop_front().unwrap();

			player.print_assets();
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
