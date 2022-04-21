use crate::cards::{Card, CardSet, PaidCardKind};
use crate::common::input;
use crate::deck::Deck;
use crate::player::{choose_card, PayWith, Player};
use crate::{game::Table, player::Assets};

/// Gets rent of `amount` from `assets`.
///
/// Returns `None` if it's not payable, else `Some` of the vector of cards the player
/// chose to pay with.
pub(crate) fn ask_for_rent(amount: u8, assets: &mut Assets) -> CardSet<PaidCardKind> {
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

		// Add the card to the vec that will be transferred to the player of the birthday card.
		cards.add(card);
	}

	cards
}

pub(crate) fn play_deal_breaker(assets: &mut Assets, table: &mut Table) {
	// Print the table.
	println!("{}", table);

	let targeted_assets = loop {
		// Let the player choose the opponent.
		match input("$ ")
			.parse::<usize>()
			.ok()
			.and_then(|n| table.get_mut_assets(n))
		{
			Some(assets) => break assets,
			None => {
				println!("Invalid index entered.");
				continue;
			}
		}
	};

	let paid_cards = ask_for_rent(5, targeted_assets);

	// FIXME Dry
	println!("5M were paid with the following cards: {}", paid_cards);

	// Add the cards paid into the assets of the player who played the birthday card.
	paid_cards.for_each(|card| assets.add_paid_card(card));
}

/// Draws two cards from the deck and adds them into the player's hand.
pub(crate) fn play_pass_go(player: &mut Player, deck: &mut Deck) {
	player.draw_two(deck);
}

/// Asks every opponent for 2M, adds the collected cards to the player's assets.
pub(crate) fn play_birthday(player_assets: &mut Assets, rest_of_the_table: &mut Table) {
	for (player, assets) in rest_of_the_table.iter_mut() {
		println!("{} needs to pay 2M.", player.name);

		// Ask the `player` to pay 2M.
		let paid_cards = ask_for_rent(2, assets);

		println!(
			"{} paid 2M with the following cards: {}",
			player.name, paid_cards
		);

		// Add the cards paid into the assets of the player who played the birthday card.
		paid_cards.for_each(|card| player_assets.add_paid_card(card));
	}
}
