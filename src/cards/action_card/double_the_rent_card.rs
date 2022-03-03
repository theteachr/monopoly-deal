#[derive(Debug, Hash, Eq, PartialEq)]
pub struct DoubleTheRentCard(u8);

impl DoubleTheRentCard {
	pub fn new(value: u8) -> Self {
		Self(value)
	}
}
