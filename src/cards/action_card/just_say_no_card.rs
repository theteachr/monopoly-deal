#[derive(Debug, Hash, Eq, PartialEq)]
pub struct JustSayNoCard(u8);

impl JustSayNoCard {
	pub fn new(value: u8) -> Self {
		Self(value)
	}
}
