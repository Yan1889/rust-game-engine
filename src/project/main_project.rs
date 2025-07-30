use crate::rust_game_engine::constants::{HEIGHT, HEIGHT_F, WIDTH, WIDTH_F};
use crate::rust_game_engine::engine_core::*;
use crate::rust_game_engine::physics::game_object::{GameObject, PhysicsAddition, PhysicsObject};
use crate::rust_game_engine::timer::Timer;
use rand::Rng;
use raylib::prelude::Vector2;

/// This function is called once when the scene is constructed
pub fn setup(default_scene: &mut Scene) {
    println!("Hello world! from setup");

    default_scene.game_objects.push(
        PhysicsObject::new(Vector2::new(100., 100.), 2000., "player".to_string())
    );

    default_scene.game_objects.extend(
        PhysicsObject::generate_walls()
    );

    default_scene.game_objects.push(
        PhysicsObject::generate_ground(Vector2::new(500., 500.))
    );
}

/// This function is called every frame and provides the delta time in s
pub fn frame(scene: &mut Scene, delta_time: f32) {
    let mut rng = rand::rng();

    if scene.mouse_clicked() {
        /*
        let mut obj: PhysicsObject = PhysicsObject::new_line(
            scene.mouse_pos(),
            scene.mouse_pos() + Vector2::new(50., 50.),
            "_".to_string(),
        );
         */
        let mut obj: PhysicsObject = PhysicsObject::new(
            scene.mouse_pos(),
            rng.random::<f32>() * 3000.,
            "_".to_string(),
        );

        if let PhysicsAddition::Dynamic {ref mut vel, ..} = obj.physics {
            *vel = Vector2::new(0., 0.);
        }
        scene.add_game_object(obj);
    }

    let key_dir: Vector2 = scene.get_key_direction();
    let main_obj: &mut PhysicsObject = scene.game_objects.first_mut().unwrap();
    main_obj.move_relative(&(key_dir * 100. * delta_time));

    if scene.timers.is_empty() {
        // spawn_one_timer(scene);
    }
}

/*
/// This is a functions provided as a demonstration how to use this engine
fn spawn_random_ball(scene: &mut Scene) {
    let pos_x: f32 = 50.;
    let pos_y: f32 = 50.;
    let mass: f32 = 10000.;
    let start_vel: Vector2 = Vector2::new(2000., -100.);

    let mut obj: PhysicsObject = PhysicsObject::new(Vector2::new(pos_x, pos_y), mass);
    if let PhysicsAddition::Dynamic {vel, ..} = obj.physics {
        *vel += start_vel;
    }
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
*/
/// This is a functions provided as a demonstration how to use this engine
fn spawn_one_timer(scene: &mut Scene) {
    let new_timer: Timer = Timer::after_seconds(scene, 1., |scene_arg: &mut Scene| {
        // spawn_random_ball(scene_arg);
        // spawn_random_square(scene_arg);
    });
    scene.timers.push(new_timer);
    println!("successfully added timer!");
}
