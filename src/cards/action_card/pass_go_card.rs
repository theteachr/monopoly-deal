use crate::{deck::Deck, player::Player};

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct PassGoCard;

impl PassGoCard {
	pub fn play(self, player: &mut Player, deck: &mut Deck) {
		player.draw_two(deck);
	}
}
