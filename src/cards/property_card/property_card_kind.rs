use std::{cmp::PartialEq, hash::Hash};

use super::{PropertyCard, PropertyWildCard};
use crate::cards::Card;
use crate::player::Assets;

/// Represents the type of a property card.
#[derive(Debug, Eq, PartialEq, Hash)]
pub enum PropertyCardKind {
	/// Holds a mono colored card.
	Single(PropertyCard),

	/// Holds a multi colored card.
	Wild(PropertyWildCard),
}

impl Card for PropertyCardKind {
	fn value(&self) -> u8 {
		match self {
			Self::Single(c) => c.value(),
			Self::Wild(c) => c.value(),
		}
	}

	fn is_playable(&self, _: &Assets) -> bool {
		true
	}
}
