mod cards;
mod color;
mod deck;
mod player;
mod game;

fn main() {
    let mut main_game = game::Game::new(4);

    let _red = player::Player::new(String::from("1415"), String::from("Red"));
    let _henry = player::Player::new(String::from("1416"), String::from("Henry"));
    let _snotty = player::Player::new(String::from("1417"), String::from("Snotty"));
    let _matilda = player::Player::new(String::from("1418"), String::from("Matilda"));

    println!("Shuffled {} cards", main_game.draw_pile.len());

    let drawn_cards = main_game.draw_pile.draw(deck::DrawCount::Two);

    println!("Drew two cards. {} left in the pile.", main_game.draw_pile.len());
    println!("{:#?}", drawn_cards);

    main_game.draw_pile.draw(deck::DrawCount::Five);
    println!("Drew five more cards. {} left in the pile.", main_game.draw_pile.len());
}
