use crate::color;

#[derive(Debug)]
pub enum CardType {
	Property(PropertyCard),
	Money(MoneyCard),
}

#[derive(Debug)]
pub struct Card {
	value: u8,
	card_type: CardType
}

#[derive(Debug)]
pub struct PropertyCard {
	title: &'static str,
	color: color::Color,
}

#[derive(Debug)]
pub struct MoneyCard;

impl Card {
	pub fn new(value: u8, card_type: CardType) -> Self {
		Card { value, card_type }
	}
}

impl PropertyCard {
	pub fn new(title: &'static str, color: color::Color) -> Self {
		PropertyCard { title, color }
	}
}
