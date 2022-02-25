use super::{
	BirthdayCard, DealBreakerCard, DebtCollectorCard, DoubleTheRentCard, ForcedDealCard, HotelCard,
	HouseCard, JustSayNoCard, PassGoCard, SlyDealCard,
};
use crate::cards::{Card, Play};
use crate::game::Turn;
use crate::player::Assets;
use std::fmt::Debug;
use std::{cmp::PartialEq, fmt, hash::Hash};

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
	fn can_play(&self, _turn: &Assets) -> bool {
		false
	}

	fn play(self, _turn: &mut Turn) {
		// TODO Ask whether to bank it or play it
		println!("Not Implemented Yet");
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
