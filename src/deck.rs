use crate::cards::*;
use crate::color::Color;

pub fn initialize_deck() -> Vec<Card> {
    let mut cards = Vec::new();

    let baltic_avenue = PropertyCard::new(String::from("Baltic Avenue"), Color::Brown);
    let mediterranean_avenue =
        PropertyCard::new(String::from("Mediterranean Avenue"), Color::Brown);

    let broadwalk = PropertyCard::new(String::from("Broadwalk"), Color::Blue);
    let park_place = PropertyCard::new(String::from("Park Place"), Color::Blue);

    let north_carolina_avenue =
        PropertyCard::new(String::from("North Carolina Avenue"), Color::Green);
    let pacific_avenue = PropertyCard::new(String::from("Pacific Avenue"), Color::Green);
    let pennsylvania_avenue = PropertyCard::new(String::from("Pennsylvania Avenue"), Color::Green);

    let connecticut_avenue =
        PropertyCard::new(String::from("Connecticut Avenue"), Color::LightBlue);
    let oriental_avenue = PropertyCard::new(String::from("Oriental Avenue"), Color::LightBlue);
    let vermont_avenue = PropertyCard::new(String::from("Vermont Avenue"), Color::LightBlue);

    let new_york_avenue = PropertyCard::new(String::from("New York Avenue"), Color::Orange);
    let st_james_place = PropertyCard::new(String::from("St. James Place"), Color::Orange);
    let tennesse_avenue = PropertyCard::new(String::from("Tennesse Avenue"), Color::Orange);

    let st_charles_place = PropertyCard::new(String::from("St. Charles Place"), Color::Pink);
    let virginia_avenue = PropertyCard::new(String::from("Virginia Avenue"), Color::Pink);
    let states_avenue = PropertyCard::new(String::from("States Avenue"), Color::Pink);

    let short_line = PropertyCard::new(String::from("Short Line"), Color::Black);
    let b_and_o_railroad = PropertyCard::new(String::from("B. & O. Railroad"), Color::Black);
    let reading_railroad = PropertyCard::new(String::from("Reading Railroad"), Color::Black);
    let pennsylvania_railroad =
        PropertyCard::new(String::from("Pennsylvania Railroad"), Color::Black);

    let kentucky_avenue = PropertyCard::new(String::from("Kentucky Avenue"), Color::Red);
    let indiana_avenue = PropertyCard::new(String::from("Indiana Avenue"), Color::Red);
    let illinois = PropertyCard::new(String::from("Illinois"), Color::Red);

    let water_works = PropertyCard::new(String::from("Water Works"), Color::LightGreen);
    let electric_company = PropertyCard::new(String::from("Electric Company"), Color::LightGreen);

    let ventor_avenue = PropertyCard::new(String::from("Ventor Avenue"), Color::Yellow);
    let marvin_gardens = PropertyCard::new(String::from("Marvin Gardens"), Color::Yellow);
    let atlantic_avenue = PropertyCard::new(String::from("Atlantic Avenue"), Color::Yellow);

    cards.push(Card::PropertyCard(baltic_avenue));

    cards
}
