use std::{
	cmp::PartialEq,
	fmt,
	hash::Hash,
};

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

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct ActionCard {
	value: u8,
	action: Action,
}

impl ActionCard {
	pub fn new(value: u8, action: Action) -> Self {
		Self { value, action }
	}
}

impl fmt::Display for ActionCard {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.action)
	}
}
