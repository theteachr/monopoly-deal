use super::{
	BirthdayCard, DealBreakerCard, DebtCollectorCard, DoubleTheRentCard, ForcedDealCard, HotelCard,
	HouseCard, JustSayNoCard, PassGoCard, SlyDealCard,
};
use crate::cards::Card;
use crate::player::Assets;
use std::fmt::Debug;
use std::{cmp::PartialEq, fmt, hash::Hash};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
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
pub enum ActionCardKind {
	Birthday(BirthdayCard),
	DealBreaker(DealBreakerCard),
	DebtCollector(DebtCollectorCard),
	DoubleTheRent(DoubleTheRentCard),
	ForcedDeal(ForcedDealCard),
	Hotel(HotelCard),
	House(HouseCard),
	JustSayNo(JustSayNoCard),
	PassGo(PassGoCard),
	SlyDeal(SlyDealCard),
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct ActionCard {
	value: u8,
	action: ActionCardKind,
}

impl ActionCard {
	pub fn new(value: u8, action: Action) -> Self {
		Self {
			value,
			action: action.into(),
		}
	}
}

impl Card for ActionCard {
	fn value(&self) -> u8 {
		self.value
	}

	fn is_playable(&self, _assets: &Assets) -> bool {
		false
	}
}

impl std::convert::From<Action> for ActionCardKind {
	fn from(action: Action) -> Self {
		match action {
			Action::Birthday => Self::Birthday(BirthdayCard),
			Action::DealBreaker => Self::DealBreaker(DealBreakerCard),
			Action::DebtCollector => Self::DebtCollector(DebtCollectorCard),
			Action::DoubleTheRent => Self::DoubleTheRent(DoubleTheRentCard),
			Action::ForcedDeal => Self::ForcedDeal(ForcedDealCard),
			Action::Hotel => Self::Hotel(HotelCard),
			Action::House => Self::House(HouseCard),
			Action::JustSayNo => Self::JustSayNo(JustSayNoCard),
			Action::PassGo => Self::PassGo(PassGoCard),
			Action::SlyDeal => Self::SlyDeal(SlyDealCard),
		}
	}
}

impl fmt::Display for ActionCard {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self.action)
	}
}

impl fmt::Display for ActionCardKind {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Birthday(c) => c.fmt(f),
			Self::DealBreaker(c) => c.fmt(f),
			Self::DebtCollector(c) => c.fmt(f),
			Self::DoubleTheRent(c) => c.fmt(f),
			Self::ForcedDeal(c) => c.fmt(f),
			Self::Hotel(c) => c.fmt(f),
			Self::House(c) => c.fmt(f),
			Self::JustSayNo(c) => c.fmt(f),
			Self::PassGo(c) => c.fmt(f),
			Self::SlyDeal(c) => c.fmt(f),
		}
	}
}
