use crate::rust_game_engine::engine_core::*;
use crate::rust_game_engine::physics::game_object::{GameObject, PhysicsObjectType};
use crate::rust_game_engine::timer::Timer;
use raylib::prelude::Vector2;

/// This function is called once when the scene is constructed
pub fn setup(default_scene: &mut Scene) {
    println!("Hello world! from setup");
}

/// This function is called every frame and provides the delta time in s
pub fn frame(scene: &mut Scene, delta_time: f32) {
    if scene.timers.is_empty() {
        spawn_one_timer(scene);
    }
}

/// This is a functions provided as a demonstration how to use this engine
fn spawn_random_ball(scene: &mut Scene) {
    let pos_x: f32 = 50.;
    let pos_y: f32 = 50.;
    let mass: f32 = 50.;
    let vel: Vector2 = Vector2::new(1000., 0.);

    let mut obj: PhysicsObjectType = PhysicsObjectType::new_ball(Vector2::new(pos_x, pos_y), mass);
    obj.get_physics_obj_mut().vel = vel;

    scene.add_game_object(obj);
}

/// This is a functions provided as a demonstration how to use this engine
fn spawn_one_timer(scene: &mut Scene) {
    let new_timer: Timer = Timer::after_seconds(scene, 0.05, |scene_arg: &mut Scene| {
        spawn_random_ball(scene_arg);
    });
    scene.timers.push(new_timer);
    println!("successfully added timer!");
}
