use macroquad::prelude::{info, warn};

const COMPLETED_LEVELS_KEY: &str = "COMPLETED_LEVELS";

pub fn save_completed_levels(completed: &[i32]) {
    info!("Saving completed levels");
    let storage = &mut quad_storage::STORAGE.lock().unwrap();
    let values: Vec<String> = completed.iter().map(|x| format!("{x}")).collect();
    let to_save = values.join(" ");

    storage.set(COMPLETED_LEVELS_KEY, &to_save);
}

pub fn load_completed_levels(level_count: usize) -> Vec<i32> {
    info!("Loading completed levels");
    let storage = &mut quad_storage::STORAGE.lock().unwrap();
    if let Some(completed_str) = storage.get(COMPLETED_LEVELS_KEY) {
        let parsed: Result<_, _> = completed_str.split(" ").map(|s| s.parse()).collect();

        let completed_levels = parsed.unwrap_or_else(|err| {
            warn!("load_completed_levels: {}", err);
            vec![]
        });

        if completed_levels.len() > level_count {
            warn!(
                "completed level count ({}) more than maximum {}",
                completed_levels.len(),
                level_count
            );

            completed_levels.into_iter().take(level_count).collect()
        } else {
            completed_levels
        }
    } else {
        info!("Completed levels not found");
        vec![]
    }
}
