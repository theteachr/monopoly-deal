use crate::color;
use crate::cards::money_card::MoneyCard;
use crate::cards::property_card::PropertyCard;

#[derive(Debug)]
pub enum CardType {
	Property(PropertyCard),
	Money(MoneyCard),
}

#[derive(Debug)]
pub struct Card {
	value: u8,
	card_type: CardType,
}

impl Card {
	pub fn new(value: u8, card_type: CardType) -> Self {
		Card { value, card_type }
	}
}
