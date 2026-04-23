use macroquad::{time::get_frame_time, window::next_frame};

use crate::{
    game::Game, level_select::LevelSelection, levels::create_levels,
    resource_manager::ResourceManager,
};

mod draw;
mod game;
mod level_select;
mod levels;
mod model;
mod physics;
mod resource_manager;

#[macroquad::main("Space Archer")]
async fn main() {
    let levels = create_levels();
    let resource_manager = ResourceManager::load();
    let mut game = Game::new(&levels, 0);
    let mut level_select = LevelSelection::new(&levels);

    loop {
        if level_select.is_in_menu {
            if let Some(selected) = level_select.draw_level_selection(&resource_manager) {
                game.set_level(selected);
            }
        } else {
            process_game_frame(&mut game, &resource_manager);
        }
        next_frame().await;
    }
}

fn process_game_frame(game: &mut Game, resource_manager: &ResourceManager) {
    const MIN_FRAME_TIME: f32 = 0.04;
    let delta = get_frame_time().min(MIN_FRAME_TIME);
    game.update(delta);
    game.draw(resource_manager);
}
