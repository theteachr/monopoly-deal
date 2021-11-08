use crate::cards::CardType::{Property, Money};
use crate::cards::{Card, MoneyCard, PropertyCard};
use crate::color::Color;

use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(Debug)]
pub struct Deck {
	cards: Vec<Card>
}

impl Deck {
	pub fn new() -> Self {
		let mut cards = Vec::new();

		// propery cards

        let property_cards_data = [
            (1, Color::Brown, vec!["Baltic Avenue", "Mediterranean Avenue"]),
            (4, Color::Blue, vec!["Broadwalk", "Park Place"]),
            (4, Color::Green, vec!["North Carolina Avenue", "Pacific Avenue", "Pennsylvania Avenue"]),
            (1, Color::LightBlue, vec!["Connecticut Avenue", "Oriental Avenue", "Vermont Avenue"]),
            (2, Color::Orange, vec!["New York Avenue", "St. James Place", "Tennesse Avenue"]),
            (2, Color::Pink, vec!["St. Charles Place", "Virginia Avenue", "States Avenue"]),
            (2, Color::Black, vec!["Short Line", "B. & O. Railroad", "Reading Railroad", "Pennsylvania Railroad"]),
            (3, Color::Red, vec!["Kentucky Avenue", "Indiana Avenue", "Illinois"]),
            (2, Color::LightGreen, vec!["Water Works", "Electric Company"]),
            (3, Color::Yellow, vec!["Ventor Avenue", "Marvin Gardens", "Atlantic Avenue"]),
        ];

        for (value, color, titles) in property_cards_data.iter() {
            for title in titles {
                cards.push(Card::new(*value, Property(PropertyCard::new(title, *color))));
            }
        }

		// actions cards

		// money cards

        let money_cards_data = [
            (10, 1),
            ( 1, 6),
            ( 2, 5),
            ( 3, 3),
            ( 4, 3),
            ( 5, 2),
        ];

        for (value, count) in money_cards_data.iter() {
            for _ in 0..*count {
                cards.push(Card::new(*value, Money(MoneyCard)))
            }
        }

        cards.shuffle(&mut thread_rng());

		Deck { cards } 
	}

	pub fn _len(&self) -> u8 {
		self.cards.len() as u8
	}
}
