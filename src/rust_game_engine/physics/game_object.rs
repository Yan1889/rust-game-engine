use raylib::prelude::*;
use crate::rust_game_engine::engine_core::*;
use crate::rust_game_engine::constants::*;

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
            accel: Vector2::new(0.0, GRAVITY),
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
        let buffer: f32 = 0.;
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