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
		let mut players = Vec::new();
		let mut draw_pile = Deck::new();

		// read players
		for id in 0..num_players {
			players.push(Player::read(id as usize));
		}

		// distribute cards
		for player in &mut players {
			player.update_hand(draw_pile.draw(DrawCount::Five));
		}

		Game {
			draw_pile,
			table: vec![(HashSet::new(), HashSet::new()); num_players as usize],
			players,
		}
	}
}
