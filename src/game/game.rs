use crate::deck::{Deck, DrawCount};

use crate::game::{
    player::Player,
    player_q::PlayerQ,
    player_state::PlayerState,
};

#[derive(Debug)]
pub struct Game<'a> {
	table: Vec<PlayerState<'a>>,
	draw_pile: Deck,
	players: PlayerQ,
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

		Self {
			draw_pile,
			table: (0..num_players).map(|_| PlayerState::new()).collect(),
			players: PlayerQ::from(players),
		}
	}

	pub fn initiate(&mut self) {
		println!("The Deal has been initiated.");

		println!("{}'s turn.", self.players.next().name);
		println!("{}'s turn.", self.players.next().name);
		println!("{}'s turn.", self.players.next().name);
		println!("{}'s turn.", self.players.next().name);
		println!("{}'s turn.", self.players.next().name);
	}
}

fn get_mock_players() -> Vec<Player> {
	["Red", "Matilda", "Bomb", "Henry"]
		.iter()
			.enumerate()
			.map(|(i, name)| Player::new(i, String::from(*name)))
			.collect()
}
