use crate::cards::money_card::MoneyCard;
use crate::cards::property_card::PropertyCard;
use crate::deck::{Deck, DrawCount};
use crate::player::Player;

use std::collections::HashSet;

#[derive(Debug)]
pub struct Game<'a> {
	pub table: Vec<(HashSet<&'a PropertyCard>, HashSet<&'a MoneyCard>)>,
	pub draw_pile: Deck,
	players: Vec<Player>,
}

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

		Game {
			draw_pile,
			table: vec![(HashSet::new(), HashSet::new()); num_players as usize],
			players,
		}
	}
}

fn get_mock_players() -> Vec<Player> {
	let names = ["Red", "Matilda", "Bomb", "Henry"];

	names
		.iter()
		.enumerate()
		.map(|(i, name)| Player::new(i, String::from(*name)))
		.collect()
}
