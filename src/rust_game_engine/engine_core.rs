use std::ops::Range;

use crate::rust_game_engine::physics::game_object::GameObject;
use raylib::prelude::MouseButton::MOUSE_BUTTON_LEFT;
use raylib::prelude::*;
use crate::rust_game_engine::timer::Timer;

pub struct Scene {
    pub game_objects: Vec<GameObject>,
    pub timers: Vec<Timer>,

    pub rl: RaylibHandle,
    pub rl_thread: RaylibThread,
}

impl Scene {
    pub fn new(rl: RaylibHandle, rl_thread: RaylibThread) -> Self {
        Self {
            game_objects: vec![],
            timers: vec![],
            rl,
            rl_thread,
        }
    }

    pub fn frame_logic(&mut self, delta_time: f32) {
        // timers
        let current_time: f32 = self.get_run_time();

        let (timers_left, timers_done): (Vec<Timer>, Vec<Timer>) = self.timers.drain(..)
            .partition(|t| current_time < t.end_time);

        for t in timers_done {
            (t.call_back)(self);
        }
        self.timers = timers_left;


        // move
        for obj in &mut self.game_objects {
            obj.update_move(delta_time);
        }

        // resolve collisions until no objects are overlapping
        let mut i: i32 = 0;
        loop {
            i += 1;
            let possible_collisions: Vec<(usize, usize)> = self.get_possible_collisions();
            let real_collisions: Vec<(usize, usize)> = self.filter_real_collisions(&possible_collisions);
            self.resolve_collisions(&real_collisions);

            if true || real_collisions.is_empty() {
                break;
            } else {
                // println!("left: {}", real_collisions.len());
                // self.render();
            }
        }
        // println!("{}", i);
    }

    pub fn render(&mut self) {
        let screen_width: i32 = self.rl.get_screen_width();
        let screen_height: i32 = self.rl.get_screen_height();
        let mut d = self.rl.begin_drawing(&self.rl_thread);

        d.clear_background(Color::WHITESMOKE);
        for obj in &self.game_objects {
            obj.render(&mut d);
        }

        d.draw_fps(screen_width - 100, screen_height - 30);
    }
    pub fn add_game_object(&mut self, game_object: GameObject) {
        self.game_objects.push(game_object);
    }

    pub fn mouse_pos(&self) -> Vector2 {
        self.rl.get_mouse_position()
    }

    pub fn mouse_clicked(&self) -> bool {
        self.rl.is_mouse_button_pressed(MOUSE_BUTTON_LEFT)
    }
    pub fn get_frame_time(&self) -> f32 {
        self.rl.get_frame_time()
    }
    pub fn get_run_time(&self) -> f32 {
        self.rl.get_time() as f32
    }

    pub fn get_random_value<T: From<i32>>(&self, num: Range<i32>) -> T {
        self.rl.get_random_value(num)
    }
}

pub fn clamp(value: &mut f32, min: f32, max: f32) {
    if *value < min {
        *value = min;
    }
    if *value > max {
        *value = max;
    }
}
