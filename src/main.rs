mod cards;
mod color;
mod common;
mod deck;
mod entities;
mod errors;
mod game;

use crate::game::Game;

fn main() {
	let mut game = Game::default();

	while game.is_running() {
		game.play();
	}
}
