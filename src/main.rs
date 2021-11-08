mod cards;
mod color;
mod deck;
mod player;
mod game;

fn main() {
    let mut main_game = game::Game::new(4);

    println!("Shuffled {} cards", main_game.draw_pile.len());

    let drawn_cards = main_game.draw_pile.draw(deck::DrawCount::Two);

    println!("Drew two cards. Cards left: {}", main_game.draw_pile.len());
    println!("{:#?}", drawn_cards);

    main_game.draw_pile.draw(deck::DrawCount::Five);
    println!("Drew five more cards. Cards left: {}", main_game.draw_pile.len());
}
