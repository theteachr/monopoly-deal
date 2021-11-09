use crate::cards::card::Card;

#[derive(Debug)]
pub struct Player {
    id: String,
    name: String,
    hand: Vec<Card>,
}

impl Player {
    pub fn new(id: String, name: String) -> Self {
        Player {
            id,
            name,
            hand: Vec::new(),
        }
    }
}
