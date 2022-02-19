use std::{cmp::PartialEq, hash::Hash};

use super::{PropertyCard, PropertyWildCard};

#[derive(Debug, Eq, Clone, Copy, PartialEq, Hash)]
pub enum PropertyCardKind {
	Single(PropertyCard),
	Wild(PropertyWildCard),
}
