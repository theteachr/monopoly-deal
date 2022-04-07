mod cards;
mod color;
mod common;
mod deck;
mod errors;
mod game;
mod player;

use game::Game;

fn main() {
	let mut deal = Game::new(4);
	
	loop {
		deal.play();
	}
}
