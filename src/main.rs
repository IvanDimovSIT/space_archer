use macroquad::{time::get_frame_time, window::next_frame};

use crate::{game::Game, levels::create_levels, resource_manager::ResourceManager};

mod draw;
mod game;
mod levels;
mod model;
mod physics;
mod resource_manager;

#[macroquad::main("Space Archer")]
async fn main() {
    let levels = create_levels();
    let resource_manager = ResourceManager::load();
    let mut game = Game::new(&levels, 0);

    loop {
        let delta = get_frame_time().min(0.04);
        game.update(delta);
        game.draw(&resource_manager);
        next_frame().await;
    }
}
