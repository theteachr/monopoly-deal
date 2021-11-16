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
	pub title: &'static str,
	pub color: Color,
	pub rents: RentVec,
}

impl PropertyCard {
	pub fn new(value: u8, title: &'static str, color: Color, rents: RentVec) -> Self {
		Self {
			value,
			title,
			color,
			rents,
		}
	}
}

impl Hash for PropertyCard {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.title.hash(state);
	}
}

impl PartialEq for PropertyCard {
	fn eq(&self, other: &Self) -> bool {
		self.title == other.title
	}
}

impl Eq for PropertyCard {}

impl fmt::Display for PropertyCard {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use Color::*;

		let text = match self.color {
			Blue => self.title.dark_blue(),
			Green => self.title.dark_green(),
			Magenta => self.title.dark_magenta(),
			Red => self.title.dark_red(),
			Yellow => self.title.dark_yellow(),
			LightBlue => self.title.blue(),
			LightGreen => self.title.green(),
			LightMagenta => self.title.magenta(),
			LightRed => self.title.red(),
			LightYellow => self.title.yellow(),
		};

		write!(f, "{}", text)
	}
}
