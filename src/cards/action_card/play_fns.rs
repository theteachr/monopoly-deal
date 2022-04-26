use crate::cards::{Card, PaidCardKind};
use crate::common::input;
use crate::deck::Deck;
use crate::player::{choose_card, PayWith, Player};
use crate::{game::Table, player::Assets};

/// Takes rent of `amount` from `from_assets` and adds them into `to_assets`.
pub(crate) fn ask_for_rent(amount: u8, from_assets: &mut Assets, to_assets: &mut Assets) {
	// Initialize the amount of value received.
	let mut paid = 0u8;

	// If `amount` is not payable by `from_assets`, transfer all cards from it to
	// `to_assets`.
	if !from_assets.can_pay(amount) {
		println!("This player is incapable of paying the rent.");

		from_assets
			.bankrupt()
			.for_each(|card| to_assets.add_paid_card(card));

		return;
	}

	// TODO Display name of the player who's being asked for rent.

	// Until the paid amount is less than the expected amount...
	while paid < amount {
		// Ask how they want to make the payment.
		let card: PaidCardKind = match choose_card(from_assets) {
			PayWith::Bank(idx) => from_assets.remove_banked_card(idx).into(),
			PayWith::Property(color) => from_assets.remove_property_card_of_color(&color).into(),
		};

		// Increase the amount paid by the value of the card.
		paid += card.value();

		// Add the card to the vec that will be transferred to the player of the birthday card.
		to_assets.add_paid_card(card);
	}
}

/// Transfers 5M worth of cards from the assets of the chosen opponent to the
/// player of the debt collector.
pub(crate) fn play_debt_collector(assets: &mut Assets, table: &mut Table) {
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

	ask_for_rent(5, targeted_assets, assets);
}

/// Draws two cards from the deck and adds them into the player's hand.
pub(crate) fn play_pass_go(player: &mut Player, deck: &mut Deck) {
	player.draw_two(deck);
}

/// Transfers 2M worth of cards from every opponent to the player of the birthday card.
pub(crate) fn play_birthday(player_assets: &mut Assets, rest_of_the_table: &mut Table) {
	for (player, assets) in rest_of_the_table.iter_mut() {
		println!("{} needs to pay 2M.", player.name);
		ask_for_rent(2, assets, player_assets);
	}
}
