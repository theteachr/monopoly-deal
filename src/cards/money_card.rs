#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[repr(u8)]
enum Denomination {
	One = 1,
	Two = 2,
	Three = 3,
	Four = 4,
	Five = 5,
	Ten = 10,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct MoneyCard {
	denomination: Denomination,
}

impl From<u8> for Denomination {
	fn from(value: u8) -> Self {
		match value {
			1 => Self::One,
			2 => Self::Two,
			3 => Self::Three,
			4 => Self::Four,
			5 => Self::Five,
			10 => Self::Ten,
			_ => unreachable!("Invalid Denomination"),
		}
	}
}

impl MoneyCard {
	pub fn new(value: u8) -> Self {
		Self {
			denomination: value.into(),
		}
	}
}
