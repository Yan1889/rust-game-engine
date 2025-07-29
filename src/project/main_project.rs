use crate::rust_game_engine::constants::{HEIGHT, HEIGHT_F, WIDTH, WIDTH_F};
use crate::rust_game_engine::engine_core::*;
use crate::rust_game_engine::physics::game_object::PhysicsObject;
use crate::rust_game_engine::timer::Timer;
use rand::Rng;
use raylib::prelude::Vector2;

/// This function is called once when the scene is constructed
pub fn setup(default_scene: &mut Scene) {
    println!("Hello world! from setup");
}

/// This function is called every frame and provides the delta time in s
pub fn frame(scene: &mut Scene, delta_time: f32) {
    if scene.mouse_clicked() {
        scene.add_game_object(PhysicsObject::new(
            scene.mouse_pos(),
            100.
        ));
    }

    if scene.timers.is_empty() {
        // spawn_one_timer(scene);
    }
}

/// This is a functions provided as a demonstration how to use this engine
fn spawn_random_ball(scene: &mut Scene) {
    let pos_x: f32 = 50.;
    let pos_y: f32 = 50.;
    let mass: f32 = 10000.;
    let vel: Vector2 = Vector2::new(2000., -100.);

    let mut obj: PhysicsObject = PhysicsObject::new(Vector2::new(pos_x, pos_y), mass);
    obj.physics.vel = vel;

    scene.add_game_object(obj);
}

/// This is a functions provided as a demonstration how to use this engine
fn spawn_random_square(scene: &mut Scene) {
    let (pos_x, pos_y) = rand::rng().random::<(f32, f32)>();
    let mass: f32 = 10000.;
    let vel: Vector2 = Vector2::new(0., 0.);

    let mut obj: PhysicsObject = PhysicsObject::new(
        Vector2::new(pos_x * WIDTH_F, pos_y * HEIGHT_F),
        mass,
    );
    obj.physics.vel = vel;

    scene.add_game_object(obj);
}

/// This is a functions provided as a demonstration how to use this engine
fn spawn_one_timer(scene: &mut Scene) {
    let new_timer: Timer = Timer::after_seconds(scene, 1., |scene_arg: &mut Scene| {
        // spawn_random_ball(scene_arg);
        spawn_random_square(scene_arg);
    });
    scene.timers.push(new_timer);
    println!("successfully added timer!");
}
