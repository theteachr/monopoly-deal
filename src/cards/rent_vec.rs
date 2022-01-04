use crate::color::CardColor;
use std::fmt;

const COLLECTIONS: [&[u8]; 10] = [
	&[1, 2, 3, 4],
	&[3, 8],
	&[1, 2],
	&[2, 4, 7],
	&[1, 2, 3],
	&[1, 2],
	&[1, 3, 5],
	&[1, 2, 4],
	&[2, 3, 6],
	&[2, 4, 6],
];

#[derive(Clone, Eq, PartialEq, Copy)]
pub struct RentVec {
	rents: &'static [u8],
}

impl RentVec {
	pub fn new(color: CardColor) -> Self {
		Self {
			rents: COLLECTIONS[color as usize],
		}
	}
}

impl fmt::Debug for RentVec {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let text = self
			.rents
			.iter()
			.map(|n| n.to_string())
			.collect::<Vec<String>>()
			.join(" ");

		write!(f, "({})", text)
	}
}
