use crate::cards::{Card, PropertySets, PaidCardKind};
use crate::common::{input, print_read_index};
use crate::deck::Deck;
use crate::errors::NotPlayable;
use crate::game::{CurrentPlayer, Game, Table};
use crate::player::{Assets, Player};

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
	for assets in rest_of_the_table.iter_mut() {
		// Initialize the amount of value received.
		let mut paid = 0u8;

		if assets.total_property_value() + assets.bank_value() < 2 {
			println!("This player is incapable of paying the rent.");
			return;
		}

		// TODO Display name of the player who's being asked for rent.

		// Until the paid amount is < 2, ask the user whether they want to play a banked or a property card.
		while paid < 2 {
			// Show the assets.
			println!("{}", assets);

			// Ask how they want to make the payment
			// `b` -> pay by a card in the bank
			// `p` -> pay by property
			let card: PaidCardKind = match input("> ").as_str() {
				"b" => {
					let idx = print_read_index("$ ", assets.bank.iter(), assets.bank.len());
					assets.remove_banked_card(idx).into()
				}
				"p" => {
					let colors = assets.property_sets.iter().collect::<Vec<_>>();
					let idx = print_read_index("# ", colors.iter(), colors.len());
					assets.remove_property_card_of_color(&colors[idx]).into()
				}
				_ => {
					println!("Press `b` to pay using a banked card and `p` to pay using a property card.");
					continue;
				}
			};

			paid += card.value();
			player_assets.add_paid_card(card);
		}
	}
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
