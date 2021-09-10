use crate::cards::*;
use crate::color::{Color, ColorSet};

pub fn initialize_deck() -> Vec<Card> {
    let mut cards = Vec::new();

    // color sets
    let brown_set = ColorSet::new(Color::Brown, 1, 2, vec![1, 2]);
    let blue_set = ColorSet::new(Color::Blue, 4, 2, vec![3, 8]);
    let green_set = ColorSet::new(Color::Green, 4, 3, vec![2, 4, 7]);
    let lightblue_set = ColorSet::new(Color::LightBlue, 1, 3, vec![1, 2, 3]);
    let orange_set = ColorSet::new(Color::Orange, 2, 3, vec![1, 3, 5]);
    let pink_set = ColorSet::new(Color::Pink, 2, 3, vec![1, 2, 4]);
    let black_set = ColorSet::new(Color::Black, 2, 4, vec![1, 2, 3, 4]);
    let red_set = ColorSet::new(Color::Red, 3, 3, vec![2, 3, 6]);
    let lightgreen_set = ColorSet::new(Color::LightGreen, 2, 4, vec![1, 2]);
    let yellow_set = ColorSet::new(Color::Yellow, 3, 2, vec![2, 4, 6]);

    let baltic_avenue = PropertyCard::new("Baltic Avenue".into(), brown_set);

    cards.push(Card::PropertyCard(baltic_avenue));

    cards
}
