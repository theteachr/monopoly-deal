/// Represents the value of a `MoneyCard`.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum Denomination {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Ten = 10,
}

impl From<u8> for Denomination {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            5 => Self::Five,
            10 => Self::Ten,
            _ => unreachable!("Invalid Denomination"),
        }
    }
}
