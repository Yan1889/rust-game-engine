use crate::rust_game_engine::constants::*;
use crate::rust_game_engine::physics::physics_addition::PhysicsAddition::*;
use crate::rust_game_engine::physics::physics_addition::*;
use rand::prelude::*;
use raylib::prelude::*;
use std::collections::HashSet;
use std::f32::consts::PI;

pub struct PhysicsObject {
    pub obj: GameObject,
    pub polygon: Polygon,
    pub physics: PhysicsAddition,
}

pub struct GameObject {
    pub pos: Vector2,
    pub rotation: f32,
    pub color: Color,
    pub name_tag: String,
}

impl PhysicsObject {
    pub fn new(pos: Vector2, mass: f32, name_tag: String) -> PhysicsObject {
        let mut rng = rand::rng();
        let color: Color = Color::new(
            rng.random::<u8>(),
            rng.random::<u8>(),
            rng.random::<u8>(),
            255,
        );

        // A ~ pi*r*r => r ~ sqrt(A / pi)
        let radius: f32 = (mass / PI).sqrt();
        let polygon: Polygon = Polygon::new_regular_polygon(pos, rng.random_range(3..=6), radius);

        PhysicsObject {
            obj: GameObject {
                pos,
                color,
                rotation: 0.,
                name_tag,
            },
            physics: Dynamic {
                vel: Vector2::zero(),
                accel: Vector2::new(0.0, GRAVITY),
                mass,
                inv_mass: 1. / mass,
            },
            polygon,
        }
    }

    pub fn generate_ground(pos: Vector2) -> PhysicsObject {
        let polygon: Polygon = Polygon::new_regular_polygon(pos, 5, 50.);

        PhysicsObject {
            obj: GameObject {
                pos,
                color: Color::RED,
                rotation: 0.,
                name_tag: "ground_obj".to_string(),
            },
            physics: Static,
            polygon,
        }
    }

    pub fn generate_walls() -> Vec<PhysicsObject> {
        let mut result: Vec<PhysicsObject> = Vec::new();

        let wall_points: [(Vector2, Vector2); 4] = [
            (Vector2::new(0., 0.), Vector2::new(0., HEIGHT_F)),
            (Vector2::new(0., HEIGHT_F), Vector2::new(WIDTH_F, HEIGHT_F)),
            (Vector2::new(WIDTH_F, HEIGHT_F), Vector2::new(WIDTH_F, 0.)),
            (Vector2::new(WIDTH_F, 0.), Vector2::new(0., 0.)),
        ];
        for (start, end) in wall_points {
            let polygon: Polygon = Polygon::new_polygon_line(start, end, 1.);
            let obj: PhysicsObject = PhysicsObject {
                obj: GameObject {
                    rotation: 0.,
                    color: Color::RED,
                    pos: (start + end) / 2.,
                    name_tag: "wall".to_string(),
                },
                physics: Static,
                polygon,
            };
            result.push(obj);
        }

        result
    }

    pub fn resolve_collision_other(&mut self, other: &mut PhysicsObject) {
        // return if none is dynamic
        if matches!((&self.physics, &other.physics), (Static, Static)) {
            return;
        }
        let Some((u_axis, overlap)) = self.get_collision_axis_and_overlap(other) else {
            return;
        };

        let (m1, m1_inv): (f32, f32) = self.physics.get_masses();
        let (m2, m2_inv): (f32, f32) = other.physics.get_masses();

        let move_percentage_self: f32 = m1_inv / (m1_inv + m2_inv);
        let move_percentage_other: f32 = m2_inv / (m1_inv + m2_inv);

        let correction_self: Vector2 = -u_axis.scale_by(move_percentage_self * overlap);
        let correction_other: Vector2 = u_axis.scale_by(move_percentage_other * overlap);

        // separate objects
        self.move_relative(&correction_self);
        other.move_relative(&correction_other);

        // update velocity
        match (self.physics.is_dynamic(), other.physics.is_dynamic()) {
            (true, true) => {
                let self_vel = self.physics.get_vel_mut().unwrap();
                let other_vel = other.physics.get_vel_mut().unwrap();

                let (v1n, v1t) = Self::unwrap_vec(self_vel, u_axis);
                let (v2n, v2t) = Self::unwrap_vec(other_vel, u_axis);

                // calculating new normal velocities (tangent remain same)
                let v1n_new: f32 = (v1n * (m1 - m2) + 2. * m2 * v2n) / (m1 + m2) * BOUNCINESS;
                let v2n_new: f32 = (v2n * (m2 - m1) + 2. * m1 * v1n) / (m1 + m2) * BOUNCINESS;

                *self_vel = Self::wrap_vec(v1n_new, v1t, u_axis);
                *other_vel = Self::wrap_vec(v2n_new, v2t, u_axis);
            }
            (true, false) => {
                let self_vel = self.physics.get_vel_mut().unwrap();

                let (v1n, v1t) = Self::unwrap_vec(self_vel, u_axis);

                let v1n_new: f32 = -v1n * BOUNCINESS;
                *self_vel = Self::wrap_vec(v1n_new, v1t, u_axis);
            }
            (false, true) => {
                let other_vel = other.physics.get_vel_mut().unwrap();

                let (v2n, v2t) = Self::unwrap_vec(other_vel, u_axis);

                let v2n_new: f32 = -v2n * BOUNCINESS;
                *other_vel = Self::wrap_vec(v2n_new, v2t, u_axis);
            }
            (false, false) => {
                unreachable!();
            }
        }
    }

    pub fn get_collision_axis_and_overlap(&self, other: &PhysicsObject) -> Option<(Vector2, f32)> {
        let mut u_axes_to_be_checked: Vec<Vector2> = Vec::new();
        u_axes_to_be_checked.extend(self.get_all_u_axes());
        u_axes_to_be_checked.extend(other.get_all_u_axes());

        let mut smallest_overlap: f32 = f32::INFINITY;
        let mut best_u_axis: Vector2 = Vector2::zero();

        let dir_self_other: Vector2 = other.obj.pos - self.obj.pos;
        for u_axis in &mut u_axes_to_be_checked {
            if u_axis.dot(dir_self_other) < 0.0 {
                u_axis.scale(-1.);
            }
            let mut self_min: f32 = f32::INFINITY;
            let mut self_max: f32 = f32::NEG_INFINITY;
            let mut other_min: f32 = f32::INFINITY;
            let mut other_max: f32 = f32::NEG_INFINITY;

            for c in &self.polygon.corners {
                let value: f32 = u_axis.dot(*c);
                self_min = self_min.min(value);
                self_max = self_max.max(value);
            }
            for c in &other.polygon.corners {
                let value: f32 = u_axis.dot(*c);
                other_min = other_min.min(value);
                other_max = other_max.max(value);
            }
            // check separating axis theorem
            if self_max < other_min || other_max < self_min {
                return None;
            }

            let overlap: f32 = f32::min(self_max, other_max) - f32::max(self_min, other_min);
            if overlap < smallest_overlap {
                smallest_overlap = overlap;
                best_u_axis = *u_axis;
            }
        }
        Some((best_u_axis, smallest_overlap))
    }
    pub fn get_all_u_axes(&self) -> Vec<Vector2> {
        let mut result: Vec<Vector2> = Vec::new();
        let corners: &Vec<Vector2> = &self.polygon.corners;
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
        let to_cell_coords = |x: f32, y: f32| -> (usize, usize) {
            let x_new: usize = (x / WIDTH_F * cell_count_x as f32) as usize;
            let y_new: usize = (y / HEIGHT_F * cell_count_y as f32) as usize;
            (x_new, y_new)
        };
        let bb: Rectangle = self.polygon.bounding_box;
        let (start_x_cell, start_y_cell) = to_cell_coords(bb.x, bb.y);
        let (end_x_cell, end_y_cell) = to_cell_coords(bb.x + bb.width, bb.y + bb.height);

        let mut result: HashSet<(usize, usize)> = HashSet::new();
        for x in start_x_cell..=end_x_cell {
            for y in start_y_cell..=end_y_cell {
                result.insert((x, y));
            }
        }
        result
    }

    pub fn unwrap_vec(vel: &Vector2, u_axis: Vector2) -> (f32, f32) {
        let u_tangent: Vector2 = Vector2::new(-u_axis.y, u_axis.x);
        (vel.dot(u_axis), vel.dot(u_tangent))
    }
    pub fn wrap_vec(n: f32, t: f32, u_axis: Vector2) -> Vector2 {
        let u_tangent: Vector2 = Vector2::new(-u_axis.y, u_axis.x);
        let n_v: Vector2 = u_axis.scale_by(n);
        let t_v: Vector2 = u_tangent.scale_by(t);
        n_v + t_v
    }

    pub fn update_move(&mut self, delta_time: f32) {
        match self.physics {
            Dynamic {
                accel, ref mut vel, ..
            } => {
                let added_vel: Vector2 = accel * delta_time;
                *vel += added_vel;

                let added_pos: Vector2 = *vel * delta_time;
                self.move_relative(&added_pos);

                let added_rotation: f32 = 0.; // 3. * delta_time;
                self.obj.rotation += added_rotation;

                // update corner rotation
                for corner in &mut self.polygon.corners {
                    let new_d_vector: Vector2 = (*corner - self.obj.pos).rotated(added_rotation);
                    *corner = self.obj.pos + new_d_vector;
                }
            }
            Static => {}
        }
    }

    pub fn move_relative(&mut self, added_pos: &Vector2) {
        self.obj.pos += *added_pos;
        self.polygon.move_relative(added_pos);
    }
    pub fn render(&self, d: &mut RaylibDrawHandle) {
        self.polygon.render(d, self.obj.color);
    }
}
