mod game;
mod input;
mod restart;

use game::Game;

fn main() {
    let mut game = Game::new();
    game.start();
}