mod cards;
mod color;
mod deck;
mod player;
mod game;

fn main() {
	let mut deal = game::Game::new(4);

	deal.initiate();
}
