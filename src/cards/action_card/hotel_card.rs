#[derive(Debug, Hash, Eq, PartialEq)]
pub struct HotelCard(u8);

impl HotelCard {
	pub fn new(value: u8) -> Self {
		Self(value)
	}
}
