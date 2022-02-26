#[derive(Debug, Hash, Eq, PartialEq)]
pub struct SlyDealCard(u8);

impl SlyDealCard {
	pub fn new(value: u8) -> Self {
		Self(value)
	}
}
