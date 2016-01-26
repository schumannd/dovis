#[macro_use]
extern crate glium;

mod dovis;

use std::thread;


fn main() {
    let lvl = dovis::MyLevel::new();

    let mut game = dovis::Game::new(lvl);

    loop {
        game.game_loop();
        thread::sleep_ms(20);
    }
}