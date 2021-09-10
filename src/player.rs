use crate::cards::*;

#[derive(Debug)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub bank: PlayerBank,
}

#[derive(Debug)]
pub struct PlayerBank {
    pub on_top: Option<MoneyCard>,
    pub rest: Vec<Card>,
}

impl Player {
    pub fn new(name: String) -> Self {
        Player {
            id: String::from("0000"),
            name,
            bank: PlayerBank {
                on_top: None,
                rest: Vec::new(),
            },
        }
    }
}
