use crate::color;
use crate::cards::sets::Set;

#[derive(Debug)]
pub struct PropertyCard {
	pub title: &'static str,
	pub color: color::Color,
	pub set: Set,
}


impl PropertyCard {
	pub fn new(title: &'static str, color: color::Color, set: Set) -> Self {
		PropertyCard { title, color, set }
	}
}
