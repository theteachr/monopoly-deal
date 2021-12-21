use std::{
	cmp::PartialEq,
	fmt,
	hash::{Hash, Hasher},
};

use crate::{
	cards::rent_vec::RentVec,
	color::{colored_text, Color},
};
use crossterm::style::Stylize;

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Card {
	Property(PropertyCard),
	Money(MoneyCard),
	Action(ActionCard),
	Wild(MultiColorCard),
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

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum Action {
	Birthday,
	DealBreaker,
	DebtCollector,
	DoubleTheRent,
	ForcedDeal,
	Hotel,
	House,
	JustSayNo,
	PassGo,
	SlyDeal,
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

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct ActionCard {
	value: u8,
	action: Action,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum MultiColor {
	Two(Color, Color),
	All,
}

#[derive(Debug, PartialEq, Hash, Eq)]
pub enum MultiColorCardType {
	Rent,
	Property,
}

#[derive(Debug, PartialEq, Hash, Eq)]
pub struct MultiColorCard {
	value: u8,
	colors: MultiColor,
	card_type: MultiColorCardType,
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
		write!(f, "{}", colored_text(self.name, self.color))
	}
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

impl ActionCard {
	pub fn new(value: u8, action: Action) -> Self {
		Self { value, action }
	}
}

impl MultiColorCard {
	pub fn new(value: u8, colors: MultiColor, card_type: MultiColorCardType) -> Self {
		Self {
			value,
			colors,
			card_type,
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
		use Card::*;

		match self {
			Property(c) => c.fmt(f),
			Money(c) => c.fmt(f),
			Action(c) => c.fmt(f),
			Wild(c) => c.fmt(f),
		}
	}
}

impl fmt::Display for ActionCard {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.action)
	}
}

impl fmt::Display for Action {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use Action::*;

		match self {
			Birthday => "Birthday",
			DealBreaker => "Deal Breaker",
			DebtCollector => "Debt Collector",
			DoubleTheRent => "Double The Rent",
			ForcedDeal => "Forced Deal",
			Hotel => "Hotel",
			House => "House",
			JustSayNo => "Just Say No",
			PassGo => "Pass Go",
			SlyDeal => "Sly Deal",
		}
		.fmt(f)
	}
}

impl fmt::Display for MultiColorCard {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use MultiColorCardType::*;

		let block = "██";
		let card_string = match self.card_type {
			Rent => "Rent",
			Property => "Property",
		};

		write!(f, "{}Card {:?}", self.colors)
	}
}
