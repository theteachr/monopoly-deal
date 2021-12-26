use std::{
	cmp::PartialEq,
	fmt,
	hash::{Hash, Hasher},
};

use crate::cards::RentVec;
use crate::color::{colored_text, Color, MultiColor};

#[derive(Debug, Eq)]
pub struct PropertyCard {
	value: u8,
	name: &'static str,
	color: Color,
	rents: RentVec,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct PropertyWildCard {
	value: u8,
	available_colors: MultiColor,
	selected_color: Option<Color>,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum EPropertyCard {
	Single(PropertyCard),
	Wild(PropertyWildCard),
}

impl PropertyWildCard {
	pub fn new(value: u8, colors: MultiColor) -> Self {
		Self {
			value,
			available_colors: colors,
			selected_color: None,
		}
	}
}

impl fmt::Display for PropertyWildCard {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let color = match self.selected_color {
			Some(c) => c,
			None => Color::Yellow, // FIXME: Use white
		};

		colored_text("Property Wild Card", color).fmt(f)
	}
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
		colored_text(self.name, self.color).fmt(f)
	}
}

impl fmt::Display for EPropertyCard {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			EPropertyCard::Single(s) => s.fmt(f),
			EPropertyCard::Wild(w) => w.fmt(f),
		}
	}
}
