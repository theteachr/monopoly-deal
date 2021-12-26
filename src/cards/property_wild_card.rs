use crate::color::MultiColor;

struct PropertyWildCard {
    colors: MultiColor
}

impl fmt::Display for Card {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PropertyWildCard {}", self.colors)
	}
}
