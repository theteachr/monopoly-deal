use crate::cards::{MultiColorCard, MultiColorCardKind};
use crate::color::MultiColor;
use std::{cmp::PartialEq, fmt, hash::Hash};

use crate::game::{Playable, Player};

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

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
pub struct ActionCard {
	value: u8,
	action: Action,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct RentCard {
	card: MultiColorCard,
}

#[derive(Debug, Eq, Copy, Clone, PartialEq, Hash)]
pub enum ActionCardKind {
	Action(ActionCard),
	Rent(RentCard),
}

impl ActionCard {
	pub fn new(value: u8, action: Action) -> Self {
		Self { value, action }
	}
}

impl RentCard {
	pub fn new(value: u8, colors: MultiColor) -> Self {
		Self {
			card: MultiColorCard::new(MultiColorCardKind::RentCard, value, colors),
		}
	}
}

impl Playable for ActionCardKind {
	fn play(self, _player: &mut Player) {
		println!("`Playable` yet to be implemented for `ActionCard`s");
	}
}

impl fmt::Display for Action {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Action::Birthday => "Birthday",
			Action::DealBreaker => "Deal Breaker",
			Action::DebtCollector => "Debt Collector",
			Action::DoubleTheRent => "Double The Rent",
			Action::ForcedDeal => "Forced Deal",
			Action::Hotel => "Hotel",
			Action::House => "House",
			Action::JustSayNo => "Just Say No",
			Action::PassGo => "Pass Go",
			Action::SlyDeal => "Sly Deal",
		}
		.fmt(f)
	}
}

impl fmt::Display for ActionCardKind {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Action(a) => a.fmt(f),
			Self::Rent(r) => r.fmt(f),
		}
	}
}

impl fmt::Display for RentCard {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.card.fmt(f)
	}
}

impl fmt::Display for ActionCard {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.action.fmt(f)
	}
}
