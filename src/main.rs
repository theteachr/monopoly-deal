mod cards;
mod color;
mod deck;
mod game;

use game::Game;

fn main() {
	let mut deal = Game::new(4);

	deal.initiate();
}
