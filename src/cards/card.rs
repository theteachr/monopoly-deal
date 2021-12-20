use std::{
	cmp::PartialEq,
	fmt,
	hash::{Hash, Hasher},
};

use crate::{cards::rent_vec::RentVec, color::Color};
use crossterm::style::Stylize;

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Card {
	Property(PropertyCard),
	Money(MoneyCard),
}

#[derive(Debug, Eq)]
pub struct PropertyCard {
	value: u8,
	name: &'static str,
	color: Color,
	rents: RentVec,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct MoneyCard {
	denomination: Denomination,
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

impl fmt::Display for MoneyCard {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}M", self.denomination as u8)
	}
}

impl fmt::Display for Card {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let string = match self {
			Card::Property(p) => p.to_string(),
			Card::Money(m) => m.to_string(),
		};

		write!(f, "{}", string)
	}
}
