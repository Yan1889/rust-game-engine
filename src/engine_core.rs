use raylib::math::Vector2;
use raylib::prelude::*;
use std::collections::{HashMap, HashSet};

use crate::{BOUNCINESS, HEIGHT_F, WIDTH_F};

pub struct Scene {
    game_objects: Vec<GameObject>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            game_objects: vec![],
        }
    }

    pub fn frame_logic(&mut self, delta_time: f32) {
        // move
        for obj in &mut self.game_objects {
            obj.update_move(delta_time);
        }

        // collisions
        self.resolve_collisions(self.filter_real_collisions(self.get_possible_collisions()));
    }

    pub fn get_possible_collisions(&self) -> Vec<(usize, usize)> {
        // collision detection - broad phase
        let obj_count = self.game_objects.len();

        // spacial partitioning - grid
        let cell_count_x: u32 = 10;
        let cell_count_y: u32 = 10;

        let mut cell_index_map: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
        // fill map
        for i in 0..obj_count {
            let obj: &GameObject = &self.game_objects[i];

            let mut cells_put_into: HashSet<(usize, usize)> = HashSet::new();
            let offsets: [(f32, f32); 4] = [(-1., -1.), (-1., 1.), (1., -1.), (1., 1.)];
            for (dx, dy) in offsets {
                let x: f32 = obj.pos.x + obj.radius * dx;
                let y: f32 = obj.pos.y + obj.radius * dy;
                let cell_coord_x: usize = (x / WIDTH_F * cell_count_x as f32) as usize;
                let cell_coord_y: usize = (y / HEIGHT_F * cell_count_y as f32) as usize;
                cells_put_into.insert((cell_coord_x, cell_coord_y));
            }

            for cell in cells_put_into {
                cell_index_map.entry(cell).or_insert(Vec::new()).push(i);
            }
        }

        let mut possible_collision_pairs: Vec<(usize, usize)> = Vec::new();

        for cell in cell_index_map {
            let obj_in_cell_count: usize = cell.1.len();
            for i in 0..obj_in_cell_count {
                for j in (i + 1)..obj_in_cell_count {
                    possible_collision_pairs.push((cell.1[i], cell.1[j]));
                }
            }
        }
        possible_collision_pairs
    }

    pub fn filter_real_collisions(
        &self,
        possible_collisions: Vec<(usize, usize)>,
    ) -> Vec<(usize, usize)> {
        // collision detection - narrow phase
        let mut real_collision_pairs: Vec<(usize, usize)> = Vec::new();
        for (i, j) in possible_collisions {
            if self.game_objects[i].collides_with(&self.game_objects[j]) {
                real_collision_pairs.push((i, j));
            }
        }
        real_collision_pairs
    }

    pub fn resolve_collisions(&mut self, collisions: Vec<(usize, usize)>) {
        for (i, j) in collisions {
            let (left, right) = self.game_objects.split_at_mut(j);
            left[i].resolve_collision_other(&mut right[0]);
        }
    }

    pub fn render(&self, d: &mut RaylibDrawHandle) {
        for obj in &self.game_objects {
            obj.render(d);
        }
    }
    pub fn add_game_object(&mut self, game_object: GameObject) {
        self.game_objects.push(game_object);
    }
}

pub struct GameObject {
    pub pos: Vector2,
    pub vel: Vector2,
    pub accel: Vector2,

    pub radius: f32,
    pub mass: f32,
}

impl GameObject {
    pub fn new(pos: Vector2, mass: f32) -> Self {
        let radius: f32 = (mass / PI as f32).sqrt();
        Self {
            pos,
            vel: Vector2::new(0.0, 0.0),
            accel: Vector2::new(0.0, 100.0),
            radius,
            mass,
        }
    }
    pub fn update_move(&mut self, delta_time: f32) {
        self.vel += self.accel * delta_time;
        self.pos += self.vel * delta_time;

        self.resolve_collision_walls();
    }
    pub fn resolve_collision_other(&mut self, other: &mut GameObject) {
        let u_normal: Vector2 = (other.pos - self.pos).normalized();
        let u_tangent: Vector2 = Vector2::new(-u_normal.y, u_normal.x);

        let v1n: f32 = self.vel.dot(u_normal);
        let v2n: f32 = other.vel.dot(u_normal);
        let v1t: f32 = self.vel.dot(u_tangent);
        let v2t: f32 = other.vel.dot(u_tangent);

        let m1: f32 = self.mass;
        let m2: f32 = other.mass;

        let v1n_new: f32 = (v1n * (m1 - m2) + 2. * m2 * v2n) / (m1 + m2) * BOUNCINESS;
        let v2n_new: f32 = (v2n * (m2 - m1) + 2. * m1 * v1n) / (m1 + m2) * BOUNCINESS;

        let v1n_new: Vector2 = u_normal.scale_by(v1n_new);
        let v2n_new: Vector2 = u_normal.scale_by(v2n_new);
        let v1t_new: Vector2 = u_tangent.scale_by(v1t);
        let v2t_new: Vector2 = u_tangent.scale_by(v2t);

        // update velocity
        self.vel = v1n_new + v1t_new;
        other.vel = v2n_new + v2t_new;

        let dist: f32 = (other.pos - self.pos).length();
        let buffer: f32 = 0.5;
        let overlap: f32 = self.radius + other.radius - dist + buffer;

        // bias
        let travel_dist_self: f32 = overlap * m1 / (m1 + m2);
        let travel_dist_other: f32 = overlap * m2 / (m1 + m2);

        let dir_self_other: Vector2 = (self.pos - other.pos).normalized(); // some buffer space

        let correction_self: Vector2 = dir_self_other.scale_by(travel_dist_self);
        let correction_other: Vector2 = -dir_self_other.scale_by(travel_dist_other);

        // separate objects
        self.pos += correction_self;
        other.pos += correction_other;
    }
    pub fn resolve_collision_walls(&mut self) {
        let left_x: f32 = self.pos.x - self.radius;
        let right_x: f32 = self.pos.x + self.radius;
        let upper_y: f32 = self.pos.y - self.radius;
        let down_y: f32 = self.pos.y + self.radius;

        clamp(&mut self.pos.x, self.radius, WIDTH_F - self.radius);
        clamp(&mut self.pos.y, self.radius, HEIGHT_F - self.radius);

        if left_x < 0. || right_x > WIDTH_F {
            self.vel.x *= -BOUNCINESS;
        }
        if upper_y < 0. || down_y > HEIGHT_F {
            self.vel.y *= -BOUNCINESS;
        }
    }
    pub fn collides_with(&self, other: &GameObject) -> bool {
        let dist_squared: f32 = (self.pos - other.pos).length_sqr();
        let radius_sum_squared: f32 = (self.radius + other.radius).powi(2);
        dist_squared < radius_sum_squared
    }
    pub fn render(&self, d: &mut RaylibDrawHandle) {
        d.draw_circle_v(&self.pos, self.radius, Color::BLACK);
    }
}
fn clamp(value: &mut f32, min: f32, max: f32) {
    if *value < min {
        *value = min;
    }
    if *value > max {
        *value = max;
    }
}
