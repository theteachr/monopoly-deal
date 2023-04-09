use super::denomination::Denomination;
use crate::cards::PropertySets;
use crate::entities::CurrentPlayer;
use crate::{cards::Card, errors::NotPlayable};
use std::{cmp::PartialEq, fmt, hash::Hash};

/// Represents a money card.
#[derive(Debug, Hash, Eq, PartialEq)]
pub struct MoneyCard {
    pub id: usize,
    pub value: Denomination,
}

impl MoneyCard {
    /// Returns a money card valued `value`.
    pub fn new(id: usize, value: Denomination) -> Self {
        Self { id, value }
    }

    pub fn play(self, current_player: &mut CurrentPlayer) {
        // Simple add the card into player's assets.
        current_player.assets.add_money(self.into());
    }
}

impl Card for MoneyCard {
    fn id(&self) -> usize {
        self.id
    }

    /// Returns the value of the card.
    fn value(&self) -> u8 {
        self.value as u8
    }

    fn is_playable(&self, _: &PropertySets) -> Result<(), NotPlayable> {
        // `MoneyCard`s are always playable.
        Ok(())
    }
}

impl fmt::Display for MoneyCard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} M", self.value())
    }
}
