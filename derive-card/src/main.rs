use derive_card::Card;

trait Card {
	fn id(&self) -> usize;
	fn value(&self) -> u8;
}

#[derive(Card, Debug)]
pub struct DumbCard {
	id: usize,
	value: u8,
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct MultiColor;

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct CardColor;

#[derive(Debug, Card, Hash, Eq, PartialEq)]
pub struct PropertyWildCard {
	pub id: usize,
	/// Represents the money the card will amount to when paid as rent.
	pub value: u8,
	/// Holds all the colors on the card.
	pub available_colors: MultiColor,
	/// Represents the current color of the card.
	pub selected_color: Option<CardColor>,
}

impl Default for PropertyWildCard {
	fn default() -> Self {
		Self {
			id: 14,
			value: 2,
			available_colors: MultiColor,
			selected_color: None,
		}
	}
}

impl Default for DumbCard {
	fn default() -> Self {
		Self { id: 4, value: 2 }
	}
}

fn main() {
	let card = PropertyWildCard::default();
	println!("{}, {}", Card::id(&card), Card::value(&card));
}
