use std::fmt;

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
