use crate::project;
use crate::rust_game_engine::constants::*;
use crate::rust_game_engine::engine_core::*;

use project::main_project::setup as user_setup;
use project::main_project::frame as user_loop;
pub fn init_game() {
    let (rl, rl_thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Elastic collisions")
        .build();

    let mut scene = Scene::new(rl, rl_thread);

    user_setup(&mut scene);

    while !scene.rl.window_should_close() {
        frame_logic(&mut scene);
        scene.render();
    }
}

fn frame_logic(scene: &mut Scene) {
    let delta_time: f32 = scene.rl.get_frame_time();

    scene.frame_logic(delta_time);
    user_loop(scene, delta_time);
}
