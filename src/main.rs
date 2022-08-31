mod cards;
mod color;
mod common;
mod deck;
mod errors;
mod entities;
mod game;

use crate::game::Game;

fn main() {
	let mut deal = Game::new(4);

	loop {
		deal.play();
	}
}
