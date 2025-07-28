# Rust game engine
## What it is
A mini physics engine made with `raylib-rs`: Create your own rigid body simulations.
## How to run
1. `git clone https://github.com/Yan1889/rust-game-engine`
2. `cd rust-game-engine && cargo run`

## How to use
Your project will be in `src/project/` by default it contains:
+ `mod.rs` (not important) 
+ `main_project.rs` this is where you can write your code, it has 2 functions:
+ + `pub fn setup(default_scene: &mut Scene)`, will be called once at in the beginning
+ + `pub fn frame(scene: &mut Scene, delta_time: f32)`, will be called every frame

With this code in `src/project/main_project.rs` you have a bouncing balls simulation  
where every 10th of a second a new ball is added:
```
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

```

### What is a Scene?
Defined in `src/rust_game_engine/engine_core.rs` and `src/rust_game_engine/physics/scene.rs`  

```
pub struct Scene {
    pub game_objects: Vec<GameObject>,
    pub timers: Vec<Timer>,

    pub rl: RaylibHandle,
    pub rl_thread: RaylibThread,
}
```