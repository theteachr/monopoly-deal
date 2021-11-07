use crate::color;

#[derive(Debug)]
pub enum CardType {
	PropertyCard(PropertyCard),
	MoneyCard(MoneyCard),
}

#[derive(Debug)]
pub struct Card {
	value: u8,
	card_type: CardType
}

#[derive(Debug)]
pub struct PropertyCard {
	title: String,
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
	pub fn new(title: String, color: color::Color) -> Self {
		PropertyCard { title, color }
	}
}
