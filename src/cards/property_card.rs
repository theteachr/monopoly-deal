use std::{
	cmp::PartialEq,
	fmt,
	hash::{Hash, Hasher},
};

use crate::color::{colored_text, CardColor, MultiColor};
use crate::common::input;
use crate::{
	cards::{MultiColorCard, MultiColorCardKind, RentVec},
	game::{Playable, Player},
};

#[derive(Debug, Clone, Copy, Eq)]
pub struct PropertyCard {
	value: u8,
	name: &'static str,
	color: CardColor,
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
	pub fn new(value: u8, name: &'static str, color: CardColor, rents: RentVec) -> Self {
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
			card: MultiColorCard::new(MultiColorCardKind::PropertyWildCard, value, colors),
		}
	}

	pub fn set_color(&mut self, color: CardColor) {
		self.card.set_color(color);
	}

	pub fn read_color(&self) -> CardColor {
		let colors = self.card.colors();
		let max_choose_num = colors.len();

		for (i, color) in colors.iter().enumerate() {
			println!("{}: {}", i, color);
		}

		// FIXME Smell -> repeating pattern of looping until
		// right input
		loop {
			if let Ok(n) = input("Choose color: ").trim().parse::<u8>() {
				if (n as usize) < max_choose_num {
					break colors[n as usize];
				}
			}

			println!(
				"Invalid color number, entered value should be between 0..={}.",
				max_choose_num
			);
		}
	}
}

impl Playable for PropertyCardKind {
	fn play(mut self, player: &mut Player) {
		if let Self::Wild(wild_card) = &mut self {
			wild_card.set_color(wild_card.read_color());
		}

		player.add_property(self);
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
