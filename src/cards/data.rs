use crate::cards::Action::{self, *};

use crate::color::{
	CardColor::{self, *},
	MultiColor,
};

//  TODO Use correct color sets
pub const PROPERTY_CARDS: [(CardColor, &[&str]); 10] = [
	(Brown, &["Baltic Avenue", "Mediterranean Avenue"]),
	(Blue, &["Broadwalk", "Park Place"]),
	(
		Green,
		&[
			"North Carolina Avenue",
			"Pacific Avenue",
			"Pennsylvania Avenue",
		],
	),
	(
		SkyBlue,
		&["Connecticut Avenue", "Oriental Avenue", "Vermont Avenue"],
	),
	(
		Orange,
		&["New York Avenue", "St. James Place", "Tennesse Avenue"],
	),
	(
		Magenta,
		&["St. Charles Place", "Virginia Avenue", "States Avenue"],
	),
	(
		Black,
		&[
			"Short Line",
			"B. & O. Railroad",
			"Reading Railroad",
			"Pennsylvania Railroad",
		],
	),
	(
		Red,
		&["Kentucky Avenue", "Indiana Avenue", "Illinois Avenue"],
	),
	(Turquoise, &["Water Works", "Electric Company"]),
	(
		Yellow,
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

pub const PROPERTY_WILD_CARDS: [(u8, MultiColor, u8); 8] = [
	(0, MultiColor::All, 2),
	(4, MultiColor::Two(Blue, Green), 1),
	(1, MultiColor::Two(Turquoise, Black), 1),
	(2, MultiColor::Two(SkyBlue, Brown), 2),
	(4, MultiColor::Two(Green, Magenta), 1),
	(4, MultiColor::Two(Turquoise, Magenta), 1),
	(2, MultiColor::Two(Orange, Magenta), 1),
	(3, MultiColor::Two(Yellow, Red), 2),
];

pub const RENT_CARDS: [(u8, MultiColor, u8); 6] = [
	(3, MultiColor::All, 3),
	(1, MultiColor::Two(Turquoise, Black), 2),
	(1, MultiColor::Two(SkyBlue, Brown), 2),
	(1, MultiColor::Two(Green, Blue), 2),
	(1, MultiColor::Two(Orange, Magenta), 2),
	(1, MultiColor::Two(Yellow, Red), 2),
];

pub const COLLECTIONS: [(u8, &[u8]); 10] = [
	(2, &[1, 2, 3, 4]), // Black
	(4, &[3, 8]),       // Blue
	(1, &[1, 2]),       // Brown
	(4, &[2, 4, 7]),    // Green
	(2, &[1, 2, 4]),    // Magenta
	(2, &[1, 3, 5]),    // Orange
	(3, &[2, 3, 6]),    // Red
	(1, &[1, 2, 3]),    // SkyBlue
	(2, &[1, 2]),       // Turquoise
	(3, &[2, 4, 6]),    // Yellow
];
