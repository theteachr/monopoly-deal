use crate::cards::{PaidCardKind, Card};
use crate::common::input;
use crate::deck::Deck;
use crate::entities::{choose_card, Assets, PaidWith, Player, Table};

/// Takes rent of `amount` from `from_assets` and adds them into `to_assets`.
pub(crate) fn ask_for_rent(
	amount: u8,
	from_assets: &mut Assets,
) -> Vec<PaidCardKind> {
	// Initialize the amount of value received.
	let mut paid = 0u8;
	let mut paid_cards = Vec::new();


	// If `amount` is not payable by `from_assets`, transfer all cards from it to
	// `to_assets`.
	if !from_assets.can_pay(amount) {
		println!("This player is incapable of paying the rent.");
		return paid_cards;
	}

	// TODO Display name of the player who's being asked for rent.

	// Until the paid amount is less than the expected amount...
	while paid < amount {
		// Ask how they want to make the payment.
		let card: PaidCardKind = match choose_card(from_assets) {
			PaidWith::Bank(id) => from_assets.bank.take(id).unwrap().into(),
			PaidWith::Property(id) => from_assets.property_sets.take_by_id(id).into(),
		};

        paid += card.value();
        paid_cards.push(card);
	}

	paid_cards
}

/// Transfers 5M worth of cards from the assets of the chosen opponent to the
/// player of the debt collector.
pub(crate) fn play_debt_collector(
	table: &mut Table,
) -> Vec<PaidCardKind> {
	// Print the table.
	println!("{}", table);

	let index = loop {
		// Let the player choose the opponent.
		match input("$ ").parse::<usize>() {
			Ok(index) if table.get_assets(index).is_some() => break index,
			_ => {
				println!("Invalid index entered.");
				continue;
			}
		}
	};

	let targeted_assets = table.get_mut_assets(index).unwrap();

	ask_for_rent(5, targeted_assets)
}

/// Draws two cards from the deck and adds them into the player's hand.
pub(crate) fn play_pass_go(player: &mut Player, deck: &mut Deck) {
	player.draw_two(deck);
}

/// Transfers 2M worth of cards from every opponent to the player of the birthday card.
pub(crate) fn play_birthday(rest_of_the_table: &mut Table) -> Vec<PaidCardKind> {
    let mut paid_cards = Vec::new();

	for (player, assets) in rest_of_the_table.iter_mut() {
		println!("{} needs to pay 2M.", player.name);
		paid_cards.extend(ask_for_rent(2, assets));
	}

    paid_cards
}
