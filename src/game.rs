use crate::deck::Deck;
use crate::cards::{MoneyCard, PropertyCard};

#[derive(Debug)]
pub struct Game {
    table: Vec<(Vec<PropertyCard>, Vec<MoneyCard>)>,
    draw_pile: Deck,
}

impl Game {
    pub fn new(num_players: u8) -> Self {
        Game { table: Vec::new(), draw_pile: Deck::new() }
    }
}
