#[derive(Debug, Hash, Eq, PartialEq)]
pub struct ForcedDealCard(u8);

impl ForcedDealCard {
	pub fn new(value: u8) -> Self {
		Self(value)
	}
}
