mod cards;
mod game;
mod deck;
mod color;

fn main() {
	let mut deal = game::game::Game::new(4);

	deal.initiate();
}
