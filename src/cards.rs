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
    color: color::Color,
}

impl PropertyCard {
    pub fn new(title: String, color: color::Color) -> Self {
        PropertyCard { title, color }
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
