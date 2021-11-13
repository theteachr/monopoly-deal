mod cards;
mod color;
mod deck;
mod game;

fn main() {
	let mut deal = game::game::Game::new(4);

	deal.initiate();
}
