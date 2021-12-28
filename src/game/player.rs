use crate::cards::{BankableCardKind, Card, CardSet, PropertyCardKind};
use crate::game::Playable;

use std::collections::HashSet;
use std::fmt;
use std::io::{stdin, stdout, Write};

pub trait Playable {
	fn play(&self, player: &mut Player);
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum PlayerAction {
	Play(u8),
	Rearrange,
}

use PlayerAction::*;

// FIXME: Increase tightness
#[derive(Debug)]
pub struct Assets {
	bank: CardSet<BankableCardKind>,
	props: CardSet<PropertyCardKind>,
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

	pub fn add_money(&mut self, card: BankableCardKind) {
		self.bank.add(card);
	}

	pub fn add_property(&mut self, card: PropertyCardKind) {
		self.props.add(card);
	}
}

#[derive(Debug)]
pub struct Player {
	pub id: usize,
	pub name: String,
	pub hand: CardSet<Card>,
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

	pub fn play(&mut self, card: Card) {
		card.play(self);
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
			// FIXME: Don't display this when the user enters nothing.
			println!("You might have an extra space between the actions.");
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
				println!(
					"Couldn't parse action: {}, should be one of (p, r).",
					action
				);
				return None;
			}
		}
	}
}

impl Playable for Card {
	fn play(&self, player: &mut Player) {
		match self {
			Card::Bankable(b) => match b {
				BankableCardKind::Money(_) => player.played.bank.add(*b),
				BankableCardKind::Action(_) => todo!(),
			},
			Card::Property(p) => player.played.props.add(*p),
		};
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
