use macroquad::{time::get_frame_time, window::next_frame};

use crate::{game::Game, levels::create_levels, resource_manager::ResourceManager};

mod draw;
mod game;
mod levels;
mod model;
mod resource_manager;

#[macroquad::main("Space Archer")]
async fn main() {
    let gameover = false;
    let levels = create_levels();
    let resource_manager = ResourceManager::load();
    let mut game = Game::new(&levels, 0);

    while !gameover {
        let delta = get_frame_time();
        game.update(delta);
        game.draw(&resource_manager);
        next_frame().await;
    }
}
