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
			     Black => self.title.dark_grey(),
			      Blue => self.title.dark_blue(),
			     Brown => self.title.grey(),
			     Green => self.title.dark_green(),
			 LightBlue => self.title.blue(),
			LightGreen => self.title.green(),
			    Orange => self.title.dark_yellow(),
			      Pink => self.title.magenta(),
			       Red => self.title.red(),
			    Yellow => self.title.yellow(),
		};

		write!(f, "{}", text)
	}
}
