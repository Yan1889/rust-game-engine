use crate::rust_game_engine::engine_core::*;
use crate::rust_game_engine::physics::game_object::GameObject;
use crate::rust_game_engine::timer::Timer;
use raylib::prelude::Vector2;

pub fn setup(default_scene: &mut Scene) {
    println!("Hello world! from setup");
}

pub fn frame(scene: &mut Scene, delta_time: f32) {
    if scene.timers.is_empty() {
        let new_timer: Timer = Timer::after_seconds(scene, 1., |scene_arg| {
            for _ in 0..10 {
                spawn_random_ball(scene_arg);
            }
        });
        scene.timers.push(new_timer);


        println!("successfully added timer!");
    }
}

fn spawn_random_ball(scene: &mut Scene) {
    let pos_x: f32 = 50.; // scene.get_random_value::<i32>(0..WIDTH) as f32;
    let pos_y: f32 = 50.; // scene.get_random_value::<i32>(0..(HEIGHT_F * 0.1) as i32) as f32;
    let mass: f32 = 200.; // scene.get_random_value::<i32>(1..1000) as f32;

    let mut obj: GameObject = GameObject::new(Vector2::new(pos_x, pos_y), mass);

    obj.vel = Vector2::new(10., 10.);
    scene.add_game_object(obj);
}
