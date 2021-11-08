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
            (1, Color::Blue, vec!["Broadwalk", "Park Place"]),
            (1, Color::Green, vec!["North Carolina Avenue", "Pacific Avenue", "Pennsylvania Avenue"]),
            (1, Color::LightBlue, vec!["Connecticut Avenue", "Oriental Avenue", "Vermont Avenue"]),
            (1, Color::Orange, vec!["New York Avenue", "St. James Place", "Tennesse Avenue"]),
            (1, Color::Pink, vec!["St. Charles Place", "Virginia Avenue", "States Avenue"]),
            (1, Color::Black, vec!["Short Line", "B. & O. Railroad", "Reading Railroad", "Pennsylvania Railroad"]),
            (1, Color::Red, vec!["Kentucky Avenue", "Indiana Avenue", "Illinois"]),
            (1, Color::LightGreen, vec!["Water Works", "Electric Company"]),
            (1, Color::Yellow, vec!["Ventor Avenue", "Marvin Gardens", "Atlantic Avenue"]),
        ];

        for (value, color, titles) in property_cards_data.iter() {
            for title in titles {
                cards.push(Card::new(*value, Property(PropertyCard::new(title, *color))));
            }
        }

		// actions cards

		// money cards

		let _ten: Card = Card::new(10, Money(MoneyCard));

        cards.shuffle(&mut thread_rng());

		Deck { cards } 
	}

	pub fn _len(&self) -> u8 {
		self.cards.len() as u8
	}
}
