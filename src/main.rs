#[macro_use]
extern crate glium;

mod dovis;

fn main() {
    let mut lvl = dovis::MyLevel{
        width: 100,
        height: 100,
        start_x: 3,
        start_y: 3,
        end_x: 95,
        end_y: 95,
        player: (3.0, 3.0),
        field: Vec::new(),
    };

    lvl.init();

    let mut game = dovis::Game::new(lvl);

    loop {
        game.game_loop();
    }
}