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