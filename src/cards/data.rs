use crate::color::Color;

pub const PROPERTIES: [(u8, Color, &[&str]); 10] = [
    (
        1,
        Color::Brown,
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
        Color::Orange,
        &["New York Avenue", "St. James Place", "Tennesse Avenue"],
    ),
    (
        2,
        Color::Pink,
        &["St. Charles Place", "Virginia Avenue", "States Avenue"],
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
    ),
    (
        3,
        Color::Red,
        &["Kentucky Avenue", "Indiana Avenue", "Illinois"],
    ),
    (
        2,
        Color::LightGreen,
        &["Water Works", "Electric Company"],
    ),
    (
        3,
        Color::Yellow,
        &["Ventor Avenue", "Marvin Gardens", "Atlantic Avenue"],
    ),
];

pub const MONIES: [(u8, u8); 6] = [(10, 1), (1, 6), (2, 5), (3, 3), (4, 3), (5, 2)];
