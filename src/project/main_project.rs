use raylib::prelude::Vector2;
use crate::rust_game_engine::constants::*;
use crate::rust_game_engine::engine_core::*;

pub fn setup(default_scene: &mut Scene) {
    println!("Hello world!");

    for _ in 0..100 {
        spawn_random_ball(default_scene);
    }
}

pub fn frame(scene: &mut Scene, delta_time: f32) {
    println!("delta time: {}", delta_time);

    if scene.mouse_clicked() {
        for _ in 0..10 {
            spawn_random_ball(scene);
        }
    }
}

fn spawn_random_ball(scene: &mut Scene) {
    let pos_x: f32 = scene.get_random_value::<i32>(0..WIDTH) as f32;
    let pos_y: f32 = scene.get_random_value::<i32>(0..(HEIGHT_F * 0.1) as i32) as f32;
    let mass: f32 = scene.get_random_value::<i32>(1..1000) as f32;

    let obj: GameObject = GameObject::new(Vector2::new(pos_x, pos_y), mass);

    scene.add_game_object(obj);
}
