#[macro_use]
extern crate glium;

mod dovis;

fn main() {
    let mut lvl = dovis::MyLevel::new();

    let mut game = dovis::Game::new(lvl);

    loop {
        game.game_loop();
    }
}