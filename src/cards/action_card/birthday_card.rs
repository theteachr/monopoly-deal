#[derive(Debug, Hash, Eq, PartialEq)]
pub struct BirthdayCard(u8);

impl BirthdayCard {
	pub fn new(value: u8) -> Self {
		Self(value)
	}
}
