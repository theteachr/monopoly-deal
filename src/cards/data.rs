use crate::cards::card::Action::{self, *};
use crate::color::Color;

pub const PROPERTY_CARDS: [(u8, Color, &[&str]); 10] = [
	(
		1,
		Color::LightRed,
		&["Baltic Avenue", "Mediterranean Avenue"],
	),
	(4, Color::Blue, &["Broadwalk", "Park Place"]),
	(
		4,
		Color::Green,
		&[
			"North Carolina Avenue",
			"Pacific Avenue",
			"Pennsylvania Avenue",
		],
	),
	(
		1,
		Color::LightBlue,
		&["Connecticut Avenue", "Oriental Avenue", "Vermont Avenue"],
	),
	(
		2,
		Color::LightYellow,
		&["New York Avenue", "St. James Place", "Tennesse Avenue"],
	),
	(
		2,
		Color::LightMagenta,
		&["St. Charles Place", "Virginia Avenue", "States Avenue"],
	),
	(
		2,
		Color::Magenta,
		&[
			"Short Line",
			"B. & O. Railroad",
			"Reading Railroad",
			"Pennsylvania Railroad",
		],
	),
	(
		3,
		Color::Red,
		&["Kentucky Avenue", "Indiana Avenue", "Illinois"],
	),
	(2, Color::LightGreen, &["Water Works", "Electric Company"]),
	(
		3,
		Color::Yellow,
		&["Ventor Avenue", "Marvin Gardens", "Atlantic Avenue"],
	),
];

pub const ACTION_CARDS: [(u8, Action, u8); 10] = [
	(2, Birthday, 3),
	(5, DealBreaker, 2),
	(3, DebtCollector, 3),
	(1, DoubleTheRent, 2),
	(3, ForcedDeal, 4),
	(4, Hotel, 3),
	(3, House, 3),
	(2, JustSayNo, 3),
	(1, PassGo, 10),
	(3, SlyDeal, 3),
];

pub const MONEY_CARDS: [(u8, u8); 6] = [(10, 1), (1, 6), (2, 5), (3, 3), (4, 3), (5, 2)];
