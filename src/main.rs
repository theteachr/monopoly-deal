mod cards;
mod color;
mod deck;
mod player;

fn main() {
    let deck = deck::Deck::new();
    let theteachr = player::Player::new(String::from("theteachr"));

    println!("{}", deck.len());
    println!("{:?}", theteachr);
}
