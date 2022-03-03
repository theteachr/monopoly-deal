#[derive(Debug, Hash, Eq, PartialEq)]
pub struct PassGoCard(u8);

impl PassGoCard {
	pub fn new(value: u8) -> Self {
		Self(value)
	}
}
