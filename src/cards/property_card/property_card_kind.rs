use std::{cmp::PartialEq, hash::Hash};

use super::{PropertyCard, PropertyWildCard};
use crate::cards::Card;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum PropertyCardKind {
	Single(PropertyCard),
	Wild(PropertyWildCard),
}

impl Card for PropertyCardKind {
	fn value(&self) -> u8 {
		match self {
			Self::Single(c) => c.value(),
			Self::Wild(c) => c.value(),
		}
	}
}
