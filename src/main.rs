mod cards;
mod color;
mod deck;
mod player;
mod game;

fn main() {
    let theteachr = player::Player::new(String::from("8100"), String::from("theteachr"));
    let game = game::Game::new(4);

    println!("{:#?}", theteachr);
    println!("{:#?}", game);
}
