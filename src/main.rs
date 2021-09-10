mod cards;
mod color;
mod deck;
mod player;

fn main() {
    let teachr = player::Player::new("theteachr".into());

    deck::initialize_deck();

    println!("{:?}", teachr);
}
