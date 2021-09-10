use crate::color;

#[derive(Debug)]
pub enum Card {
    PropertyCard(PropertyCard),
    ActionCard(ActionCard),
    RentCard(RentCard),
    MoneyCard(MoneyCard),
    PropertyWildCard(PropertyWildCard),
}

#[derive(Debug)]
pub struct MoneyCard {
    value: u8,
}

#[derive(Debug)]
pub struct ActionCard {
    value: u8,
}

#[derive(Debug)]
pub struct PropertyCard {
    title: String,
    color_set: color::ColorSet,
}

impl PropertyCard {
    pub fn new(title: String, color_set: color::ColorSet) -> Self {
        PropertyCard { title, color_set }
    }
}

#[derive(Debug)]
pub struct PropertyWildCard {
    value: u8,
}

#[derive(Debug)]
pub struct RentCard {
    value: u8,
}
