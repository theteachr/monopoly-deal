use crate::deck::Deck;
use crate::cards::money_card::MoneyCard;
use crate::cards::property_card::PropertyCard;

#[derive(Debug)]
pub struct Game<'a> {
	pub table: Vec<(Vec<&'a PropertyCard>, Vec<&'a MoneyCard>)>,
	pub draw_pile: Deck,
}

impl Game<'_> {
	pub fn new(num_players: u8) -> Self {
		Game {
			draw_pile: Deck::new(),
			table: vec![(Vec::new(), Vec::new()); num_players as usize],
		}
	}
}
