use crate::cards::sets::*;
use crate::color::Color;

pub const PROPERTIES: [(u8, Color, &[&str], &[u8]); 10] = [
    (
        1,
        Color::Brown,
        &["Baltic Avenue", "Mediterranean Avenue"],
        &BROWN_SET,
    ),
    (4, Color::Blue, &["Broadwalk", "Park Place"], &BLUE_SET),
    (
        4,
        Color::Green,
        &[
            "North Carolina Avenue",
            "Pacific Avenue",
            "Pennsylvania Avenue",
        ],
        &GREEN_SET,
    ),
    (
        1,
        Color::LightBlue,
        &["Connecticut Avenue", "Oriental Avenue", "Vermont Avenue"],
        &LIGHTBLUE_SET,
    ),
    (
        2,
        Color::Orange,
        &["New York Avenue", "St. James Place", "Tennesse Avenue"],
        &ORANGE_SET,
    ),
    (
        2,
        Color::Pink,
        &["St. Charles Place", "Virginia Avenue", "States Avenue"],
        &PINK_SET,
    ),
    (
        2,
        Color::Black,
        &[
            "Short Line",
            "B. & O. Railroad",
            "Reading Railroad",
            "Pennsylvania Railroad",
        ],
        &BLACK_SET,
    ),
    (
        3,
        Color::Red,
        &["Kentucky Avenue", "Indiana Avenue", "Illinois"],
        &RED_SET,
    ),
    (
        2,
        Color::LightGreen,
        &["Water Works", "Electric Company"],
        &LIGHTGREEN_SET,
    ),
    (
        3,
        Color::Yellow,
        &["Ventor Avenue", "Marvin Gardens", "Atlantic Avenue"],
        &YELLOW_SET,
    ),
];

pub const MONIES: [(u8, u8); 6] = [(10, 1), (1, 6), (2, 5), (3, 3), (4, 3), (5, 2)];
