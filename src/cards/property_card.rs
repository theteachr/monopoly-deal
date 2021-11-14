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
		let text = match self.color {
			Color::Black	   => self.title.dark_grey(),
			Color::Blue		   => self.title.dark_blue(),
			Color::Brown	   => self.title.grey(),
			Color::Green	   => self.title.dark_green(),
			Color::LightBlue   => self.title.blue(),
			Color::LightGreen  => self.title.green(),
			Color::Orange	   => self.title.dark_yellow(),
			Color::Pink		   => self.title.magenta(),
			Color::Red		   => self.title.red(),
			Color::Yellow	   => self.title.yellow(),
		};

		write!(f, "{}", text)
	}
}
