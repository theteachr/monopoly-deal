use super::{
	BirthdayCard, DealBreakerCard, DebtCollectorCard, DoubleTheRentCard, ForcedDealCard, HotelCard,
	HouseCard, JustSayNoCard, PassGoCard, SlyDealCard,
};
use crate::cards::{Card, Play};
use crate::game::Turn;
use crate::player::Assets;
use std::fmt::Debug;
use std::{cmp::PartialEq, fmt, hash::Hash};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum Action {
	Birthday(u8),
	DealBreaker(u8),
	DebtCollector(u8),
	DoubleTheRent(u8),
	ForcedDeal(u8),
	Hotel(u8),
	House(u8),
	JustSayNo(u8),
	PassGo(u8),
	SlyDeal(u8),
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

impl Card for ActionCardKind {
	fn value(&self) -> u8 {
		0
	}
}

impl Play for ActionCardKind {
	fn is_playable(&self, _turn: &Assets) -> bool {
		false
	}

	fn play(self, _turn: &mut Turn) {
		// TODO Ask whether to bank it or play it
		println!("Not Implemented Yet");
	}
}

impl std::convert::From<Action> for ActionCardKind {
	fn from(action: Action) -> Self {
		match action {
			Action::Birthday(v) => Self::Birthday(BirthdayCard::new(v)),
			Action::DealBreaker(v) => Self::DealBreaker(DealBreakerCard::new(v)),
			Action::DebtCollector(v) => Self::DebtCollector(DebtCollectorCard::new(v)),
			Action::DoubleTheRent(v) => Self::DoubleTheRent(DoubleTheRentCard::new(v)),
			Action::ForcedDeal(v) => Self::ForcedDeal(ForcedDealCard::new(v)),
			Action::Hotel(v) => Self::Hotel(HotelCard::new(v)),
			Action::House(v) => Self::House(HouseCard::new(v)),
			Action::JustSayNo(v) => Self::JustSayNo(JustSayNoCard::new(v)),
			Action::PassGo(v) => Self::PassGo(PassGoCard::new(v)),
			Action::SlyDeal(v) => Self::SlyDeal(SlyDealCard::new(v)),
		}
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
