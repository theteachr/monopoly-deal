use crate::color;

#[derive(Debug)]
pub struct PropertyCard {
	pub title: &'static str,
	pub color: color::Color,
	pub set: &'static [u8],
}


impl PropertyCard {
	pub fn new(title: &'static str, color: color::Color, set: &'static [u8]) -> Self {
		PropertyCard { title, color, set }
	}
}
