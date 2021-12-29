use std::{
	cmp::PartialEq,
	fmt,
	hash::{Hash, Hasher},
};

use crate::color::{colored_text, Color, MultiColor};
use crate::common::input;
use crate::{
	cards::{MultiColorCard, RentVec},
	game::{Playable, Player},
};

#[derive(Debug, Clone, Copy, Eq)]
pub struct PropertyCard {
	value: u8,
	name: &'static str,
	color: Color,
	rents: RentVec,
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
pub struct PropertyWildCard {
	card: MultiColorCard,
}

#[derive(Debug, Eq, Clone, Copy, PartialEq, Hash)]
pub enum PropertyCardKind {
	Single(PropertyCard),
	Wild(PropertyWildCard),
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

impl PropertyWildCard {
	pub fn new(value: u8, colors: MultiColor) -> Self {
		Self {
			card: MultiColorCard::new("PropertyWildCard", value, colors),
		}
	}

	pub fn set_color(&mut self, color: Color) {
		self.card.set_color(color);
	}

	pub fn read_color(&self) -> Color {
		let colors = self.card.colors();

		for (i, color) in colors.iter().enumerate() {
			println!("{}: {:?}", i, color);
		}

		// FIXME: Smell -> repeating pattern of looping until
		// right input
		loop {
			if let Ok(n) = input("Choose color: ").trim().parse::<u8>() {
				break colors[n as usize];
			}

			println!("Invalid color number, please try again.");
			continue;
		}
	}
}

impl Playable for PropertyCard {
	fn play(self, player: &mut Player) {
		player.played.add_property(PropertyCardKind::Single(self));
	}
}

impl Playable for PropertyWildCard {
	fn play(mut self, player: &mut Player) {
		let color_chosen = self.read_color();
		self.set_color(color_chosen);
		player.played.add_property(PropertyCardKind::Wild(self));
	}
}

impl Playable for PropertyCardKind {
	fn play(self, player: &mut Player) {
		match self {
			Self::Single(s) => s.play(player),
			Self::Wild(w) => w.play(player),
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
		write!(
			f,
			"{} ({}) {:?}",
			colored_text(self.name, self.color),
			self.value,
			self.rents
		)
	}
}

impl fmt::Display for PropertyWildCard {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.card.fmt(f)
	}
}

impl fmt::Display for PropertyCardKind {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Single(s) => s.fmt(f),
			Self::Wild(w) => w.fmt(f),
		}
	}
}
