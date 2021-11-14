use crate::cards::money_card::MoneyCard;
use crate::cards::property_card::PropertyCard;

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Card {
	Property(PropertyCard),
	Money(MoneyCard),
}
