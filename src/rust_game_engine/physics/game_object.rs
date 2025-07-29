use crate::rust_game_engine::constants::*;
use rand::prelude::*;
use raylib::prelude::*;
use std::collections::HashSet;
use std::f32::consts::TAU;

pub struct PhysicsObject {
    pub obj: GameObject,
    pub physics: PhysicsAddition,
}

pub struct GameObject {
    pub pos: Vector2,
    pub rotation: f32,
    pub color: Color,
}
pub struct PhysicsAddition {
    pub accel: Vector2,
    pub vel: Vector2,

    pub corners: Vec<Vector2>,
    pub mass: f32,
}

impl PhysicsObject {
    pub fn new(pos: Vector2, mass: f32) -> PhysicsObject {
        let mut rng = rand::rng();
        let color: Color = Color::new(
            rng.random::<u8>(),
            rng.random::<u8>(),
            rng.random::<u8>(),
            255,
        );

        let radius: f32 = 50.;
        let mut corners: Vec<Vector2> = Vec::new();

        // regular polygon
        // pentagon
        let corner_count: usize = 3;
        for i in 0..corner_count {
            let angle: f32 = i as f32 / corner_count as f32 * TAU;
            let vector_relative: Vector2 = Vector2::new(0., 1.)
                .scale_by(radius)
                .rotated(angle);
            corners.push(pos + vector_relative);
        }

        PhysicsObject {
            obj: GameObject {
                pos,
                color,
                rotation: 0.,
            },
            physics: PhysicsAddition {
                vel: Vector2::zero(),
                accel: Vector2::new(0.0, GRAVITY),
                corners,
                mass,
            },
        }
    }

    pub fn resolve_collision_other(&mut self, other: &mut PhysicsObject) {
        /*
        let u_normal: Vector2 = (other_obj.pos - self_obj.pos).normalized();
        let u_tangent: Vector2 = Vector2::new(-u_normal.y, u_normal.x);

        let v1n: f32 = self_physics.vel.dot(u_normal);
        let v2n: f32 = other_physics.vel.dot(u_normal);
        let v1t: f32 = self_physics.vel.dot(u_tangent);
        let v2t: f32 = other_physics.vel.dot(u_tangent);

        let m1: f32 = self_physics.mass;
        let m2: f32 = other_physics.mass;

        let v1n_new: f32 = (v1n * (m1 - m2) + 2. * m2 * v2n) / (m1 + m2) * BOUNCINESS;
        let v2n_new: f32 = (v2n * (m2 - m1) + 2. * m1 * v1n) / (m1 + m2) * BOUNCINESS;

        let v1n_new: Vector2 = u_normal.scale_by(v1n_new);
        let v2n_new: Vector2 = u_normal.scale_by(v2n_new);
        let v1t_new: Vector2 = u_tangent.scale_by(v1t);
        let v2t_new: Vector2 = u_tangent.scale_by(v2t);

        // update velocity
        self_physics.vel = v1n_new + v1t_new;
        other_physics.vel = v2n_new + v2t_new;

        let dist: f32 = (other_obj.pos - self_obj.pos).length();
        let overlap: f32 = *self_radius + *other_radius - dist;

        // bias
        let travel_dist_self: f32 = overlap * m1 / (m1 + m2);
        let travel_dist_other: f32 = overlap * m2 / (m1 + m2);

        let dir_self_other: Vector2 = (self_obj.pos - other_obj.pos).normalized();

        let correction_self: Vector2 = dir_self_other.scale_by(travel_dist_self);
        let correction_other: Vector2 = -dir_self_other.scale_by(travel_dist_other);

        // separate objects
        self_obj.pos += correction_self;
        other_obj.pos += correction_other;
         */

        let mut u_axes_to_be_checked: Vec<Vector2> = Vec::new();
        u_axes_to_be_checked.extend(self.get_all_u_axes());
        u_axes_to_be_checked.extend(other.get_all_u_axes());

        let mut best_u_axis: &Vector2 = &Default::default();
        let mut best_dist: f32 = f32::INFINITY;

        for u_axis in &u_axes_to_be_checked {
            let mut self_min: f32 = f32::INFINITY;
            let mut self_max: f32 = f32::NEG_INFINITY;
            let mut other_min: f32 = f32::INFINITY;
            let mut other_max: f32 = f32::NEG_INFINITY;

            let self_corners: &Vec<Vector2> = &self.physics.corners;
            let other_corners: &Vec<Vector2> = &other.physics.corners;
            for &c in self_corners {
                let value: f32 = u_axis.dot(c);
                self_min = self_min.min(value);
                self_max = self_max.max(value);
            }
            for &c in other_corners {
                let value: f32 = u_axis.dot(c);
                other_min = other_min.min(value);
                other_max = other_max.max(value);
            }

            let resolution_dist_1: f32 = self_max - other_min;
            let resolution_dist_2: f32 = other_max - self_min;
            let real_dist: f32 = if resolution_dist_1.abs() < resolution_dist_2.abs() {
                resolution_dist_1
            } else {
                resolution_dist_2
            };
            if real_dist < best_dist.abs() {
                best_dist = real_dist;
                best_u_axis = u_axis;
            }
        }

        let m1: f32 = self.physics.mass;
        let m2: f32 = other.physics.mass;

        let travel_dist_self: f32 = best_dist * m1 / (m1 + m2);
        let travel_dist_other: f32 = best_dist * m2 / (m1 + m2);

        let correction_self: Vector2 = best_u_axis.scale_by(travel_dist_self);
        let correction_other: Vector2 = -best_u_axis.scale_by(travel_dist_other);

        // separate objects
        self.move_relative(&correction_self);
        other.move_relative(&correction_other);
    }

    pub fn resolve_collision_walls(&mut self) {
        todo!()
    }

    pub fn collides_with(&self, other: &PhysicsObject) -> bool {
        let mut u_axes_to_be_checked: Vec<Vector2> = Vec::new();
        u_axes_to_be_checked.extend(self.get_all_u_axes());
        u_axes_to_be_checked.extend(other.get_all_u_axes());

        for u_axis in u_axes_to_be_checked {
            let mut self_min: f32 = f32::INFINITY;
            let mut self_max: f32 = f32::NEG_INFINITY;
            let mut other_min: f32 = f32::INFINITY;
            let mut other_max: f32 = f32::NEG_INFINITY;

            let self_corners: &Vec<Vector2> = &self.physics.corners;
            let other_corners: &Vec<Vector2> = &other.physics.corners;
            for &c in self_corners {
                let value: f32 = u_axis.dot(c);
                self_min = self_min.min(value);
                self_max = self_max.max(value);
            }
            for &c in other_corners {
                let value: f32 = u_axis.dot(c);
                other_min = other_min.min(value);
                other_max = other_max.max(value);
            }
            // check separating axis theorem
            if self_max < other_min || other_max < self_min {
                return false;
            }
        }
        true
    }

    pub fn get_all_u_axes(&self) -> Vec<Vector2> {
        let mut result: Vec<Vector2> = Vec::new();
        let corners: &Vec<Vector2> = &self.physics.corners;
        for i in 0..corners.len() {
            let c1: Vector2 = corners[i];
            let c2: Vector2 = corners[(i + 1) % corners.len()];
            let normal: Vector2 = c2 - c1;
            let u_tangent: Vector2 = Vector2::new(-normal.y, normal.x).normalized();
            result.push(u_tangent);
        }
        result
    }

    pub fn get_cell_positions(
        &self,
        (cell_count_x, cell_count_y): (usize, usize),
    ) -> HashSet<(usize, usize)> {
        let mut cells_put_into: HashSet<(usize, usize)> = HashSet::new();
        for corner in &self.physics.corners {
            let cell_coord_x: usize = (corner.x / WIDTH_F * cell_count_x as f32) as usize;
            let cell_coord_y: usize = (corner.y / HEIGHT_F * cell_count_y as f32) as usize;
            cells_put_into.insert((cell_coord_x, cell_coord_y));
        }

        cells_put_into
    }

    pub fn update_move(&mut self, delta_time: f32) {
        let added_vel: Vector2 = self.physics.accel * delta_time;
        self.physics.vel += added_vel;

        let added_pos: Vector2 = self.physics.vel * delta_time;
        self.move_relative(&added_pos);

        let added_rotation: f32 = 0.; // 3. * delta_time;
        self.obj.rotation += added_rotation;


        // update corner rotation
        for corner in &mut self.physics.corners {
            let new_d_vector: Vector2 = (*corner - self.obj.pos).rotated(added_rotation);
            *corner = self.obj.pos + new_d_vector;
        }

        self.resolve_collision_walls();
    }

    pub fn move_relative(&mut self, added_pos: &Vector2) {
        self.obj.pos += *added_pos;
        for corner in &mut self.physics.corners {
            *corner += *added_pos;
        }
    }
    pub fn render(&self, d: &mut RaylibDrawHandle) {
        let corner_count: usize = self.physics.corners.len();
        for i in 0..corner_count {
            let first_corner: &Vector2 = &self.physics.corners[i];
            let second_corner: &Vector2 = &self.physics.corners[(i + 1) % corner_count];
            d.draw_line_ex(first_corner, second_corner, 5., self.obj.color);
        }
    }
}
