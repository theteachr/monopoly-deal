use crate::cards::action::Action;

use crate::color::{
    CardColor::{self, *},
    MultiColor,
};

type NumCards = u8;
type Rents = [u8];
type Value = u8;
type PropertyNames = &'static [&'static str];

pub const PROPERTY_CARDS: [(CardColor, PropertyNames); 10] = [
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

pub const ACTION_CARDS: [(Value, Action, NumCards); 10] = [
    (2, Action::Birthday, 3),
    (5, Action::DealBreaker, 2),
    (3, Action::DebtCollector, 3),
    (1, Action::DoubleTheRent, 2),
    (3, Action::ForcedDeal, 4),
    (4, Action::Hotel, 3),
    (3, Action::House, 3),
    (2, Action::JustSayNo, 3),
    (1, Action::PassGo, 10),
    (3, Action::SlyDeal, 3),
];

pub const MONEY_CARDS: [(Value, NumCards); 6] = [(10, 1), (1, 6), (2, 5), (3, 3), (4, 3), (5, 2)];

pub const PROPERTY_WILD_CARDS: [(Value, MultiColor, NumCards); 8] = [
    (0, MultiColor::All, 2),
    (4, MultiColor::Two(Blue, Green), 1),
    (1, MultiColor::Two(Turquoise, Black), 1),
    (2, MultiColor::Two(SkyBlue, Brown), 2),
    (4, MultiColor::Two(Green, Magenta), 1),
    (4, MultiColor::Two(Turquoise, Magenta), 1),
    (2, MultiColor::Two(Orange, Magenta), 1),
    (3, MultiColor::Two(Yellow, Red), 2),
];

pub const RENT_CARDS: [(Value, MultiColor, NumCards); 6] = [
    (3, MultiColor::All, 3),
    (1, MultiColor::Two(Turquoise, Black), 2),
    (1, MultiColor::Two(SkyBlue, Brown), 2),
    (1, MultiColor::Two(Green, Blue), 2),
    (1, MultiColor::Two(Orange, Magenta), 2),
    (1, MultiColor::Two(Yellow, Red), 2),
];

pub const COLLECTIONS: [(Value, &Rents); 10] = [
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
