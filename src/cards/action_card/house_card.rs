#[derive(Debug, Hash, Eq, PartialEq)]
pub struct HouseCard(u8);

impl HouseCard {
	pub fn new(value: u8) -> Self {
		Self(value)
	}
}
