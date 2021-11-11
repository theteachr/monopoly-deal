use crate::color::Color;
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

#[derive(Clone, Copy)]
pub struct Set {
	set: &'static [u8],
}

impl Set {
	pub fn new(color: Color) -> Self {
		Self {
			set: COLLECTIONS[color as usize],
		}
	}
}

impl fmt::Debug for Set {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let text = self
			.set
			.iter()
			.map(|n| n.to_string())
			.collect::<Vec<String>>()
			.join(" ");

		write!(f, "({})", text)
	}
}
