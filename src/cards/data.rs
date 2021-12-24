use crate::cards::Action::{self, *};

use crate::color::{
	Color::{self, *},
	MultiColor,
};

//  Brown   LightRed
//  Orange  LightYellow
//  Pink    LightMagenta
//  Black   Magenta

pub const PROPERTY_CARDS: [(u8, Color, &[&str]); 10] = [
	(1, LightRed, &["Baltic Avenue", "Mediterranean Avenue"]),
	(4, Blue, &["Broadwalk", "Park Place"]),
	(
		4,
		Green,
		&[
			"North Carolina Avenue",
			"Pacific Avenue",
			"Pennsylvania Avenue",
		],
	),
	(
		1,
		LightBlue,
		&["Connecticut Avenue", "Oriental Avenue", "Vermont Avenue"],
	),
	(
		2,
		LightYellow,
		&["New York Avenue", "St. James Place", "Tennesse Avenue"],
	),
	(
		2,
		LightMagenta,
		&["St. Charles Place", "Virginia Avenue", "States Avenue"],
	),
	(
		2,
		Magenta,
		&[
			"Short Line",
			"B. & O. Railroad",
			"Reading Railroad",
			"Pennsylvania Railroad",
		],
	),
	(3, Red, &["Kentucky Avenue", "Indiana Avenue", "Illinois"]),
	(2, LightGreen, &["Water Works", "Electric Company"]),
	(
		3,
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
	(1, MultiColor::Two(LightBlue, LightRed), 1),
	(2, MultiColor::Two(LightYellow, LightMagenta), 2),
	(4, MultiColor::Two(Green, Magenta), 1),
	(4, MultiColor::Two(LightBlue, Magenta), 1),
	(2, MultiColor::Two(LightGreen, Magenta), 1),
	(3, MultiColor::Two(Yellow, Red), 2),
];

pub const RENT_CARDS: [(u8, MultiColor, u8); 6] = [
	(3, MultiColor::All, 3),
	(1, MultiColor::Two(LightBlue, LightRed), 2),
	(1, MultiColor::Two(LightYellow, LightMagenta), 2),
	(1, MultiColor::Two(Green, Blue), 2),
	(1, MultiColor::Two(LightGreen, Magenta), 2),
	(1, MultiColor::Two(Yellow, Red), 2),
];
