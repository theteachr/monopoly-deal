use std::{
    collections::{HashMap, HashSet},
    fmt,
};

use crate::cards::{data::COLLECTIONS, Card};
use crate::cards::{CardSet, PropertyCardKind};
use crate::color::CardColor;

/// Tracks the set of property cards played by a player.
#[derive(Debug)]
pub struct PropertySets {
    /// A map from color to set of cards, where the value holds all cards of
    /// that color.
    properties: HashMap<CardColor, CardSet<PropertyCardKind>>,
    /// A list of colors that are complete sets.
    completed_colors: Vec<CardColor>,
}

// TODO Add logic to track color sets.

impl PropertySets {
    /// Returns an empty set of properties.
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
            completed_colors: Vec::new(),
        }
    }

    /// Adds the card into the respective color bucket. If it makes a complete set after adding, adds the color
    /// into the vec tracking the number of sets.
    pub fn add(&mut self, card: PropertyCardKind) {
        let color = match card {
            PropertyCardKind::Single(ref c) => c.color,
            PropertyCardKind::Wild(ref c) => c.selected_color.unwrap(),
        };

        // Insert the card into map with key as `color`.
        self.properties
            .entry(color)
            .or_insert_with(CardSet::new)
            .add(card);
        // Get the number of cards for a complete set for the `color`. If it equals the current count,
        // add the color to `completed_colors` to indicate the completion of the set.
        if self.properties[&color].len() == num_cards_for_complete_set(color) {
            self.completed_colors.push(color);
        }
    }

    /// Returns all colors in the properties.
    pub fn colors(&self) -> HashSet<CardColor> {
        self.properties.keys().cloned().collect()
    }

    pub fn properties(&self) -> impl Iterator<Item = &PropertyCardKind> {
        self.properties.values().flat_map(|cards| cards.iter())
    }

    /// Pops a card from the `color` set.
    pub fn pop(&mut self, property_card: &PropertyCardKind) -> Option<PropertyCardKind> {
        let color = match property_card {
            PropertyCardKind::Single(card) => card.color,
            PropertyCardKind::Wild(_) => return None,
        };

        let cards = self.properties.get_mut(&color).unwrap();
        let popped = cards.take(property_card.id()).unwrap();

        if cards.is_empty() {
            self.properties.remove(&color);
        }

        Some(popped)
    }

    /// Returns `true` if `color` is a complete set.
    pub fn is_complete_set(&self, color: CardColor) -> bool {
        self.properties
            .get(&color)
            .map(|cards| cards.len())
            .unwrap_or(0)
            == num_cards_for_complete_set(color)
    }

    pub fn take_by_id(&mut self, id: usize) -> PropertyCardKind {
        let (_, mut cards) = self
            .properties
            .iter_mut()
            .find(|(_, cards)| cards.get(id).is_some())
            .unwrap();

        cards.take(id).unwrap()
    }

    pub fn take(&mut self, card: &PropertyCardKind) -> Option<PropertyCardKind> {
        let color = match card {
            PropertyCardKind::Single(c) => c.color,
            PropertyCardKind::Wild(c) => c.selected_color.unwrap(),
        };

        self.properties
            .get_mut(&color)
            .and_then(|cards| cards.take(card.id()))
    }

    /// Returns the amount of rent that the player will be paid,
    /// if they choose to ask rent for all their `color` properties.
    pub fn rent(&self, color: CardColor) -> u8 {
        self.properties
            .get(&color)
            .map_or(0, |cards| rent_for(color, cards.len()))
    }

    /// Returns the total value all the played properties.
    pub fn total_value(&self) -> u8 {
        self.properties
            .values()
            .flat_map(|collection| collection.iter().map(Card::value))
            .sum()
    }

    /// Returns `true` if at least one property of the given `color` exists in the set.
    pub fn exists(&self, color: &CardColor) -> bool {
        self.properties.contains_key(color)
    }

    /// Returns an iterator over the colors played by the players.
    pub fn iter(&self) -> impl Iterator<Item = CardColor> + '_ {
        self.properties.keys().cloned()
    }

    /// Returns all properties and empties the player property assets.
    pub fn go_popper(&mut self) -> Vec<PropertyCardKind> {
        todo!();
    }

    pub fn property_cards_of_color(&self, color: CardColor) -> Option<&CardSet<PropertyCardKind>> {
        self.properties.get(&color)
    }
}

impl fmt::Display for PropertySets {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.properties.is_empty() {
            return write!(f, "[]");
        }

        let text = self
            .properties
            .iter()
            .map(|(color, cards)| format!("{}: {:?}", color, cards))
            .collect::<Vec<String>>()
            .join("\n  ");

        write!(f, "[\n  {}\n]", text)
    }
}

/// Returns the array of rents for the `color`.
fn rents(color: CardColor) -> &'static [u8] {
    COLLECTIONS[color as usize].1
}

/// Returns the amount of rent the player will be paid for `num_cards` of `color`.
fn rent_for(color: CardColor, num_cards: usize) -> u8 {
    rents(color)[(num_cards - 1) as usize]
}

/// Returns the number of cards required for a complete set for the `color`.
fn num_cards_for_complete_set(color: CardColor) -> usize {
    rents(color).len() as usize
}
