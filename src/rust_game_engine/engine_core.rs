use std::collections::HashSet;
use std::ops::Range;
use raylib::ffi::KeyboardKey::KEY_LEFT;
use crate::rust_game_engine::physics::game_object::{PhysicsObject};
use crate::rust_game_engine::timer::Timer;
use raylib::prelude::MouseButton::MOUSE_BUTTON_LEFT;
use raylib::prelude::*;
use raylib::prelude::KeyboardKey::{KEY_DOWN, KEY_RIGHT, KEY_UP};

pub struct Scene {
    pub timers: Vec<Timer>,
    pub game_objects: Vec<PhysicsObject>,
    pub space_partitioning_grid_size: (usize, usize),

    pub rl: RaylibHandle,
    pub rl_thread: RaylibThread,
}

impl Scene {
    pub fn new(rl: RaylibHandle, rl_thread: RaylibThread) -> Self {
        Self {
            timers: vec![],
            game_objects: vec![],
            space_partitioning_grid_size: (10, 10),
            rl,
            rl_thread,
        }
    }

    pub fn frame_logic(&mut self, delta_time: f32) {
        // timers
        let current_time: f32 = self.get_run_time();

        let (timers_left, timers_done): (Vec<Timer>, Vec<Timer>) = self.timers
            .drain(..)
            .partition(|t| current_time < t.end_time);

        for t in timers_done {
            (t.callback)(self);
        }
        self.timers = timers_left;


        // move
        for obj in &mut self.game_objects {
            obj.update_move(delta_time);
        }


        let possible_collisions: HashSet<(usize, usize)> = self.get_possible_collisions();
        let real_collisions: Vec<(usize, usize)> = self.filter_real_collisions(possible_collisions);
        self.resolve_collisions(&real_collisions);

        /*
        for obj in &mut self.game_objects {
            obj.obj.color = Color::BLUE;
        }

        for &(i, j) in &possible_collisions {
            self.game_objects[i].obj.color = Color::ORANGE;
            self.game_objects[j].obj.color = Color::ORANGE;
        }
        for &(i, j) in &real_collisions {
            self.game_objects[i].obj.color = Color::RED;
            self.game_objects[j].obj.color = Color::RED;
        }
         */
    }

    // todo
    // pub fn get_object_by_name_tag_mut(&mut self, name_tag: &str) -> Option<&mut PhysicsObject> {}
    pub fn get_first_object_mut(&mut self) -> &mut PhysicsObject {
        self.game_objects.first_mut().unwrap()
    }

    pub fn render(&mut self) {
        let screen_width: i32 = self.rl.get_screen_width();
        let screen_height: i32 = self.rl.get_screen_height();
        let grid_dimensions: &(usize, usize) = &self.space_partitioning_grid_size;

        let display_info: Vec<String> = self.get_display_info();

        let mut d = self.rl.begin_drawing(&self.rl_thread);
        d.clear_background(Color::WHITESMOKE);

        // display objects
        for obj in &self.game_objects {
            obj.render(&mut d);
        }
        // display grid
        for i in 0..grid_dimensions.0 {
            let y: f32 = i as f32 / grid_dimensions.0 as f32 * screen_height as f32;
            d.draw_line(0, y as i32, screen_width, y as i32, Color::RED);
        }
        for i in 0..grid_dimensions.1 {
            let x: f32 = i as f32 / grid_dimensions.1 as f32 * screen_width as f32;
            d.draw_line(x as i32, 0, x as i32, screen_height, Color::RED);
        }

        // display info text
        for i in 0..display_info.len() {
            let s: &str = &display_info[i];
            d.draw_text(s, screen_width - 300, i as i32 * 40, 30, Color::DARKBLUE);
        }
        // display fps
        d.draw_fps(screen_width - 100, screen_height - 30);
    }

    pub fn get_display_info(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        result.push(format!("Object count: {}", self.game_objects.len()));
        result.push(format!("Grid size: {:?}", self.space_partitioning_grid_size));
        result
    }
    pub fn add_game_object(&mut self, game_object: PhysicsObject) {
        self.game_objects.push(game_object);
    }

    pub fn mouse_pos(&self) -> Vector2 {
        self.rl.get_mouse_position()
    }

    pub fn get_key_direction(&self) -> Vector2 {
        let mut result: Vector2 = Vector2::zero();
        if self.rl.is_key_down(KEY_LEFT) {
            result += Vector2::new(-1., 0.);
        }
        if self.rl.is_key_down(KEY_RIGHT) {
            result += Vector2::new(1., 0.);
        }
        if self.rl.is_key_down(KEY_UP) {
            result += Vector2::new(0., -1.);
        }
        if self.rl.is_key_down(KEY_DOWN) {
            result += Vector2::new(0., 1.);
        }
        result
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

pub fn clamp(value: &mut f32, min: &f32, max: &f32) {
    *value = value.min(*max).max(*min);
}
