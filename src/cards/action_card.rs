use crate::cards::multi_color_card::RentCard;
use std::{cmp::PartialEq, fmt, hash::Hash};

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

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct ActionCard {
	value: u8,
	action: Action,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum ActionCardKind {
	Action(ActionCard),
	Rent(RentCard),
}

impl ActionCard {
	pub fn new(value: u8, action: Action) -> Self {
		Self { value, action }
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

impl fmt::Display for ActionCard {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.action.fmt(f)
	}
}
