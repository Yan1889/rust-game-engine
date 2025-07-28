use crate::rust_game_engine::engine_core::Scene;


pub struct Timer {
    pub start_time: f32,
    pub end_time: f32,
    pub callback: fn(scene: &mut Scene),
}

impl Timer {
    pub fn after_seconds(scene: &mut Scene, x: f32, callback: fn(scene: &mut Scene)) -> Timer {
        let start_time: f32 = scene.get_run_time();
        let end_time: f32 = start_time + x;
        Timer {
            start_time,
            end_time,
            callback,
        }
    }
}