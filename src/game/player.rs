use crate::cards::{Card, CardSet};

use std::collections::HashSet;
use std::fmt;
use std::io::{stdin, stdout, Write};

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum PlayerAction {
	Play(u8),
	Rearrange,
}

use PlayerAction::*;

#[derive(Debug)]
pub struct Assets {
	bank: CardSet,
	props: CardSet,
}

impl fmt::Display for Assets {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Bank: {} Properties: {}", self.bank, self.props)
	}
}

impl Assets {
	pub fn new() -> Self {
		Self {
			bank: CardSet::new(),
			props: CardSet::new(),
		}
	}

	// XXX: Use a different name other than `len` because it usually returns a `usize`
	// or make it return a `usize`
	pub fn len(&self) -> u8 {
		self.bank.len() + self.props.len()
	}

	pub fn add(&mut self, card: Card) {
		let mut slot = match card {
			Card::Money(_) => &mut self.bank,
			Card::Property(_) => &mut self.props,
			_ => unreachable!(),
		};

		slot.add(card);
	}
}

#[derive(Debug)]
pub struct Player {
	pub id: usize,
	pub name: String,
	pub hand: CardSet,
	pub played: Assets,
}

impl Player {
	pub fn new(id: usize, name: String) -> Self {
		Self {
			id,
			name,
			hand: CardSet::new(),
			played: Assets::new(),
		}
	}

	pub fn update_hand(&mut self, cards: Vec<Card>) {
		for card in cards {
			self.hand.add(card);
		}
	}

	pub fn play_card_at(&mut self, position: usize) {
		let selected_card = self.hand.remove(position);

		println!("Played {}.", selected_card);
		self.played.add(selected_card);
	}

	pub fn play_cards_at(&mut self, mut card_positions: Vec<u8>) {
		card_positions.sort_by_key(|k| std::cmp::Reverse(*k));

		for pos in card_positions {
			self.play_card_at(pos.into());
		}
	}

	pub fn print_assets(&self) {
		println!("{}'s assets: {}", self.name, self.played);
	}

	pub fn print_hand(&self) {
		println!("{}'s hand: {}", self.name, self.hand);
	}

	pub fn print_numbered_hand(&self) {
		println!("{}'s hand:", self.name);

		for (i, card) in self.hand.cards().iter().enumerate() {
			println!("{}: {}", i, card);
		}
	}

	pub fn read_actions(&self) -> Vec<PlayerAction> {
		loop {
			match input("> ")
				.trim()
				.split(" ")
				.map(|s| self.process_action_str(s))
				.collect::<Option<HashSet<PlayerAction>>>()
			{
				Some(actions) => break actions.into_iter().collect(),
				_ => continue,
			}
		}
	}

	fn process_action_str(&self, action: &str) -> Option<PlayerAction> {
		if action.is_empty() {
			return None;
		}

		match (&action[0..1], &action[1..].parse::<u8>()) {
			("p", Ok(n)) => {
				if *n >= self.hand.len() {
					println!(
						"Invalid card number: {}, should be in {}..={}.",
						n,
						0,
						self.hand.len() - 1
					);

					return None;
				}
				return Some(Play(*n));
			}
			("r", _) => Some(Rearrange),
			_ => {
				println!("Couldn't parse action: {}, should be one of (p, r)", action);
				return None;
			}
		}
	}
}

fn input(prompt: &str) -> String {
	let mut input = String::new();

	print!("{}", prompt);
	stdout().flush().expect("Couldn't flush :<");

	stdin()
		.read_line(&mut input)
		.expect("Couldn't read from `stdin` :<");

	return input;
}
