use raylib::prelude::*;
use raylib::prelude::MouseButton::MOUSE_BUTTON_LEFT;

mod engine_core;
use engine_core::*;

pub const WIDTH: i32 = 1080;
pub const HEIGHT: i32 = 720;
pub const WIDTH_F: f32 = 1080.;
pub const HEIGHT_F: f32 = 720.;
pub const BOUNCINESS: f32 = 0.9;

fn main() {
    let (mut rl, thread) = init().size(WIDTH, HEIGHT).title("Elastic collisions").build();

    // rl.set_target_fps(60);

    let mut scene = Scene::new();

    for _ in 0..100 {
        spawn_random_ball(&mut rl, &mut scene);
    }

    while !rl.window_should_close() {
        frame_logic(&mut rl, &mut scene);
        frame_drawing(&mut rl, &thread, &mut scene);
    }
}

fn frame_logic(rl: &mut RaylibHandle, scene: &mut Scene) {
    let delta_time: f32 = rl.get_frame_time();
    scene.frame_logic(delta_time);

    if rl.is_mouse_button_pressed(MOUSE_BUTTON_LEFT) {
        for _ in 0..100 {
            spawn_random_ball(rl, scene);
        }
    }
}

fn spawn_random_ball(rl: &mut RaylibHandle, scene: &mut Scene) {
    let pos_x: f32 = rl.get_random_value::<i32>(0..WIDTH) as f32;
    let pos_y: f32 = rl.get_random_value::<i32>(0..(HEIGHT_F * 0.1) as i32) as f32;
    let vel_x: f32 = rl.get_random_value::<i32>(-100..100) as f32;
    let vel_y: f32 = rl.get_random_value::<i32>(-100..100) as f32;
    let mass: f32 = rl.get_random_value::<i32>(1..1000) as f32;

    let mut obj: GameObject = GameObject::new(
        Vector2::new(pos_x, pos_y),
        mass
    );
    // obj.vel = Vector2::new(vel_x, vel_y);

    scene.add_game_object(obj);
}

fn frame_drawing(rl: &mut RaylibHandle, thread: &RaylibThread, scene: &mut Scene) {
    let mut d = rl.begin_drawing(&thread);
    d.clear_background(Color::WHITE);

    scene.render(&mut d);

    d.draw_fps(WIDTH - 100, HEIGHT - 30);
}
