use crate::color;

#[derive(Debug)]
pub struct PropertyCard {
	title: &'static str,
	color: color::Color,
}


impl PropertyCard {
	pub fn new(title: &'static str, color: color::Color) -> Self {
		PropertyCard { title, color }
	}
}
