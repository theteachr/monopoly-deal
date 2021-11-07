use crate::cards::*;
use crate::color::Color;

pub struct Deck {
	cards: Vec<Card>
}

impl Deck {
	pub fn new() -> Self {
		let mut cards = Vec::new();

		// propery cards

		let baltic_avenue = Card::new(1, CardType::PropertyCard(PropertyCard::new(String::from("Baltic Avenue"), Color::Brown)));
		let mediterranean_avenue = Card::new(1, CardType::PropertyCard(PropertyCard::new(String::from("Mediterranean Avenue"), Color::Brown)));
		let broadwalk = Card::new(1, CardType::PropertyCard(PropertyCard::new(String::from("Broadwalk"), Color::Blue)));
		let park_place = Card::new(1, CardType::PropertyCard(PropertyCard::new(String::from("Park Place"), Color::Blue)));
		let north_carolina_avenue = Card::new(1, CardType::PropertyCard(PropertyCard::new(String::from("North Carolina Avenue"), Color::Green)));
		let pacific_avenue = Card::new(1, CardType::PropertyCard(PropertyCard::new(String::from("Pacific Avenue"), Color::Green)));
		let pennsylvania_avenue = Card::new(1, CardType::PropertyCard(PropertyCard::new(String::from("Pennsylvania Avenue"), Color::Green)));
		let connecticut_avenue = Card::new(1, CardType::PropertyCard(PropertyCard::new(String::from("Connecticut Avenue"), Color::LightBlue)));
		let oriental_avenue = Card::new(1, CardType::PropertyCard(PropertyCard::new(String::from("Oriental Avenue"), Color::LightBlue)));
		let vermont_avenue = Card::new(1, CardType::PropertyCard(PropertyCard::new(String::from("Vermont Avenue"), Color::LightBlue)));
		let new_york_avenue = Card::new(1, CardType::PropertyCard(PropertyCard::new(String::from("New York Avenue"), Color::Orange)));
		let st_james_place = Card::new(1, CardType::PropertyCard(PropertyCard::new(String::from("St. James Place"), Color::Orange)));
		let tennesse_avenue = Card::new(1, CardType::PropertyCard(PropertyCard::new(String::from("Tennesse Avenue"), Color::Orange)));
		let st_charles_place = Card::new(1, CardType::PropertyCard(PropertyCard::new(String::from("St. Charles Place"), Color::Pink)));
		let virginia_avenue = Card::new(1, CardType::PropertyCard(PropertyCard::new(String::from("Virginia Avenue"), Color::Pink)));
		let states_avenue = Card::new(1, CardType::PropertyCard(PropertyCard::new(String::from("States Avenue"), Color::Pink)));
		let short_line = Card::new(1, CardType::PropertyCard(PropertyCard::new(String::from("Short Line"), Color::Black)));
		let b_and_o_railroad = Card::new(1, CardType::PropertyCard(PropertyCard::new(String::from("B. & O. Railroad"), Color::Black)));
		let reading_railroad = Card::new(1, CardType::PropertyCard(PropertyCard::new(String::from("Reading Railroad"), Color::Black)));
		let pennsylvania_railroad = Card::new(1, CardType::PropertyCard(PropertyCard::new(String::from("Pennsylvania Railroad"), Color::Black)));
		let kentucky_avenue = Card::new(1, CardType::PropertyCard(PropertyCard::new(String::from("Kentucky Avenue"), Color::Red)));
		let indiana_avenue = Card::new(1, CardType::PropertyCard(PropertyCard::new(String::from("Indiana Avenue"), Color::Red)));
		let illinois = Card::new(1, CardType::PropertyCard(PropertyCard::new(String::from("Illinois"), Color::Red)));
		let water_works = Card::new(1, CardType::PropertyCard(PropertyCard::new(String::from("Water Works"), Color::LightGreen)));
		let electric_company = Card::new(1, CardType::PropertyCard(PropertyCard::new(String::from("Electric Company"), Color::LightGreen)));
		let ventor_avenue = Card::new(1, CardType::PropertyCard(PropertyCard::new(String::from("Ventor Avenue"), Color::Yellow)));
		let marvin_gardens = Card::new(1, CardType::PropertyCard(PropertyCard::new(String::from("Marvin Gardens"), Color::Yellow)));
		let atlantic_avenue = Card::new(1, CardType::PropertyCard(PropertyCard::new(String::from("Atlantic Avenue"), Color::Yellow)));

		// actions cards

		// money cards

		let ten = Card::new(10, CardType::MoneyCard(MoneyCard));

		cards.push(baltic_avenue);
		cards.push(mediterranean_avenue);
		cards.push(broadwalk);
		cards.push(park_place);
		cards.push(north_carolina_avenue);
		cards.push(pacific_avenue);
		cards.push(pennsylvania_avenue);
		cards.push(connecticut_avenue);
		cards.push(oriental_avenue);
		cards.push(vermont_avenue);
		cards.push(new_york_avenue);
		cards.push(st_james_place);
		cards.push(tennesse_avenue);
		cards.push(st_charles_place);
		cards.push(virginia_avenue);
		cards.push(states_avenue);
		cards.push(short_line);
		cards.push(b_and_o_railroad);
		cards.push(reading_railroad);
		cards.push(pennsylvania_railroad);
		cards.push(kentucky_avenue);
		cards.push(indiana_avenue);
		cards.push(illinois);
		cards.push(water_works);
		cards.push(electric_company);
		cards.push(ventor_avenue);
		cards.push(marvin_gardens);
		cards.push(atlantic_avenue);

		cards.push(ten);

		Deck { cards } 
	}

	pub fn len(&self) -> u8 {
		self.cards.len() as u8
	}
}
