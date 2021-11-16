use std::fmt;

use crate::cards::rent_vec::RentVec;
use crate::color::Color;

use std::{
	cmp::PartialEq,
	hash::{Hash, Hasher},
};

use crossterm::style::Stylize;

#[derive(Debug)]
pub struct PropertyCard {
	value: u8,
	pub name: &'static str,
	pub color: Color,
	pub rents: RentVec,
}

impl PropertyCard {
	pub fn new(value: u8, name: &'static str, color: Color, rents: RentVec) -> Self {
		Self {
			value,
			name,
			color,
			rents,
		}
	}
}

impl Hash for PropertyCard {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.name.hash(state);
	}
}

impl PartialEq for PropertyCard {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name
	}
}

impl Eq for PropertyCard {}

impl fmt::Display for PropertyCard {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use Color::*;

		let text = match self.color {
			Blue => self.name.dark_blue(),
			Green => self.name.dark_green(),
			Magenta => self.name.dark_magenta(),
			Red => self.name.dark_red(),
			Yellow => self.name.dark_yellow(),
			LightBlue => self.name.blue(),
			LightGreen => self.name.green(),
			LightMagenta => self.name.magenta(),
			LightRed => self.name.red(),
			LightYellow => self.name.yellow(),
		};

		write!(f, "{}", text)
	}
}
