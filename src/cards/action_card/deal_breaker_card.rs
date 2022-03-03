#[derive(Debug, Hash, Eq, PartialEq)]
pub struct DealBreakerCard(u8);

impl DealBreakerCard {
	pub fn new(value: u8) -> Self {
		Self(value)
	}
}
