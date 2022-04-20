use crate::cards::{Card, CardSet, PaidCardKind, PropertySets};
use crate::deck::Deck;
use crate::errors::NotPlayable;
use crate::game::{CurrentPlayer, Game, Table};
use crate::player::{choose_card, Assets, PayWith, Player};

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
pub struct ActionCard {
	pub value: u8,
	pub action: Action,
}

impl ActionCard {
	pub fn new(value: u8, action: Action) -> Self {
		Self { value, action }
	}

	pub fn play(self, player: &mut CurrentPlayer, game: &mut Game) {
		match self.action {
			Action::PassGo => play_pass_go(player.get(), &mut game.deck),
			Action::Birthday => play_birthday(&mut player.assets, &mut game.table),
			_ => todo!(),
		}

		game.discard_deck.push_back(self.into());
	}
}

/// Draws two cards from the deck and adds them into the player's hand.
fn play_pass_go(player: &mut Player, deck: &mut Deck) {
	player.draw_two(deck);
}

/// Asks every opponent for 2M, adds the collected cards to the player's assets.
fn play_birthday(player_assets: &mut Assets, rest_of_the_table: &mut Table) {
	for (player, assets) in rest_of_the_table.iter_mut() {
		println!("{} needs to pay 2M.", player.name);

		// Ask the `player` to pay 2M.
		let cards = ask_for_rent(2, assets);

		println!(
			"{} paid 2M with the following cards: {}",
			player.name, cards
		);

		// Add the cards paid into the assets of the player who played the birthday card.
		cards.for_each(|card| player_assets.add_paid_card(card));
	}
}

/// Gets rent of `amount` from `assets`.
///
/// Returns `None` if it's not payable, else `Some` of the vector of cards the player
/// chose to pay with.
fn ask_for_rent(amount: u8, assets: &mut Assets) -> CardSet<PaidCardKind> {
	// Initialize the amount of value received.
	let mut paid = 0u8;

	// Hold the cards used to pay the rent.
	let mut cards = CardSet::<PaidCardKind>::new();

	// If the total amount the player can pay is less than the requested,
	// take all cards from the player.
	if !assets.can_pay(amount) {
		println!("This player is incapable of paying the rent.");
		return assets.bankrupt();
	}

	// TODO Display name of the player who's being asked for rent.

	// Until the paid amount is < 2, ask the user whether they want to play a banked or a property card.
	while paid < amount {
		// Ask how they want to make the payment
		let card: PaidCardKind = match choose_card(assets) {
			PayWith::Bank(idx) => assets.remove_banked_card(idx).into(),
			PayWith::Property(color) => assets.remove_property_card_of_color(&color).into(),
		};

		// Increase the amount paid by the value of the card.
		paid += card.value();

		// Add the card to the vec that will transferred to the player of the birthday card.
		cards.add(card);
	}

	cards
}

impl Card for ActionCard {
	fn value(&self) -> u8 {
		self.value
	}

	fn is_playable(&self, _properties: &PropertySets) -> Result<(), NotPlayable> {
		match self.action {
			Action::PassGo | Action::Birthday | Action::DealBreaker | Action::DebtCollector => {
				Ok(())
			}
			Action::DoubleTheRent => Err(NotPlayable(
				"You need to play a rent card before playing this one.".to_string(),
			)),
			Action::ForcedDeal => Ok(()),
			Action::Hotel => Ok(()),
			Action::House => Ok(()),
			Action::JustSayNo => Err(NotPlayable(
				"Can only be played when you're asked to pay or to counter another JustSayNo."
					.to_string(),
			)),
			Action::SlyDeal => Ok(()),
		}
	}
}

impl fmt::Display for ActionCard {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.action.fmt(f)
	}
}
