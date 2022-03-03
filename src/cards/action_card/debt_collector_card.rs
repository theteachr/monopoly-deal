#[derive(Debug, Hash, Eq, PartialEq)]
pub struct DebtCollectorCard(u8);

impl DebtCollectorCard {
	pub fn new(value: u8) -> Self {
		Self(value)
	}
}
