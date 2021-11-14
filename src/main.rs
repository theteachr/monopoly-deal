mod cards;
mod color;
mod deck;
mod game;

fn main() {
	let mut deal = game::Game::new(2);

	deal.initiate();
}
