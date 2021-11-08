use crate::deck::Deck;
use crate::cards::{MoneyCard, PropertyCard};

#[derive(Debug)]
pub struct Game<'a> {
    table: Vec<(Vec<&'a PropertyCard>, Vec<&'a MoneyCard>)>,
    draw_pile: Deck,
}

impl Game<'_> {
    pub fn new(num_players: u8) -> Self {
        Game { table: vec![(Vec::new(), Vec::new()); num_players as usize], draw_pile: Deck::new() }
    }
}
