use crate::rust_game_engine::constants::*;
use crate::rust_game_engine::physics::game_object::PhysicsAddition::{Dynamic, Static};
use rand::prelude::*;
use raylib::prelude::*;
use std::collections::HashSet;
use std::f32::consts::{PI, TAU};

pub struct PhysicsObject {
    pub obj: GameObject,
    pub physics: PhysicsAddition,
}

pub struct GameObject {
    pub pos: Vector2,
    pub rotation: f32,
    pub color: Color,
    pub name_tag: String,
}
pub enum PhysicsAddition {
    Dynamic {
        accel: Vector2,
        vel: Vector2,
        corners: Vec<Vector2>,
        mass: f32,
    },
    Static {
        corners: Vec<Vector2>,
    },
}

impl PhysicsAddition {
    pub fn generate_regular_polygon(corner_count: usize, radius: f32) -> Vec<Vector2> {
        let mut corners: Vec<Vector2> = Vec::new();
        for i in 0..corner_count {
            let angle: f32 = i as f32 / corner_count as f32 * TAU + PI;
            let vector_relative: Vector2 = Vector2::new(0., 1.).scale_by(radius).rotated(angle);
            corners.push(vector_relative);
        }
        corners
    }
    pub fn get_corners(&self) -> &Vec<Vector2> {
        match self {
            Dynamic { corners, .. } => corners,
            Static { corners, .. } => corners,
        }
    }
    pub fn get_corners_mut(&mut self) -> &mut Vec<Vector2> {
        match self {
            Dynamic {
                ref mut corners, ..
            } => corners,
            Static {
                ref mut corners, ..
            } => corners,
        }
    }

    pub fn get_mass(&self) -> f32 {
        match self {
            Dynamic { mass, .. } => *mass,
            Static { .. } => f32::INFINITY,
        }
    }
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
        let mut corners: Vec<Vector2> =
            PhysicsAddition::generate_regular_polygon(rng.random_range(3..10), radius);

        for c in &mut corners {
            *c += pos;
        }

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
                corners,
                mass,
            },
        }
    }

    pub fn generate_ground(pos: Vector2) -> PhysicsObject {
        let mut corners: Vec<Vector2> = PhysicsAddition::generate_regular_polygon(5, 50.);

        for c in &mut corners {
            *c += pos;
        }

        PhysicsObject {
            obj: GameObject {
                pos,
                color: Color::BLACK,
                rotation: 0.,
                name_tag: "ground_obj".to_string(),
            },
            physics: Static { corners },
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
            let obj: PhysicsObject = PhysicsObject {
                obj: GameObject {
                    rotation: 0.,
                    color: Color::BLACK,
                    pos: (start + end) / 2.,
                    name_tag: "wall".to_string(),
                },
                physics: Static {
                    corners: vec![start, end],
                },
            };
            result.push(obj);
        }

        result
    }

    pub fn resolve_collision_other(&mut self, other: &mut PhysicsObject) {
        let collision_result = self.get_collision_axis_and_overlap(other);
        if collision_result.is_none() {
            return;
        }
        let (mut best_u_axis, overlap) = collision_result.unwrap();

        let self_other_dir: Vector2 = other.obj.pos - self.obj.pos;
        if best_u_axis.dot(self_other_dir) < 0.0 {
            best_u_axis.scale(-1.);
        }

        let m1: f32 = if let Dynamic { mass, .. } = &self.physics {
            *mass
        } else {
            0.
        };
        let m2: f32 = if let Dynamic { mass, .. } = &other.physics {
            *mass
        } else {
            0.
        };

        let m1_inverse: f32 = if m1 == 0. { 0. } else { 1. / m1 };
        let m2_inverse: f32 = if m2 == 0. { 0. } else { 1. / m2 };
        let m_inverse_sum: f32 = m1_inverse + m2_inverse;

        let move_percentage_self: f32 = if m_inverse_sum == 0. {
            0.
        } else {
            m1_inverse / m_inverse_sum
        };
        let move_percentage_other: f32 = if m_inverse_sum == 0. {
            0.
        } else {
            m2_inverse / m_inverse_sum
        };

        let correction_self: Vector2 = -best_u_axis.scale_by(move_percentage_self * overlap);
        let correction_other: Vector2 = best_u_axis.scale_by(move_percentage_other * overlap);

        // separate objects
        self.move_relative(&correction_self);
        other.move_relative(&correction_other);

        // update velocity
        /*
        let u_tangent: Vector2 = Vector2::new(-best_u_axis.y, best_u_axis.x);

        let mut v1n: f32 = 0.;
        let mut v1t: f32 = 0.;
        let mut v2n: f32 = 0.;
        let mut v2t: f32 = 0.;

        if let Dynamic { ref mut vel, .. } = self.physics {
            v1n = vel.dot(best_u_axis);
            v1t = vel.dot(u_tangent);
        }
        if let Dynamic { ref mut vel, .. } = other.physics {
            v2n = vel.dot(best_u_axis);
            v2t = vel.dot(u_tangent);
        }

        let v1n_new: f32 = (v1n * (m1 - m2) + 2. * m2 * v2n) / (m1 + m2) * BOUNCINESS;
        let v1t_new: f32 = v1t;
        let v2n_new: f32 = (v2n * (m2 - m1) + 2. * m1 * v1n) / (m1 + m2) * BOUNCINESS;
        let v2t_new: f32 = v2t;

        let v1n_new_v: Vector2 = best_u_axis.scale_by(v1n_new);
        let v1t_new_v: Vector2 = u_tangent.scale_by(v1t_new);
        let v2n_new_v: Vector2 = best_u_axis.scale_by(v2n_new);
        let v2t_new_v: Vector2 = u_tangent.scale_by(v2t_new);

        if let Dynamic { ref mut vel, .. } = self.physics {
            *vel = v1n_new_v + v1t_new_v;
        }
        if let Dynamic { ref mut vel, .. } = other.physics {
            *vel = v2n_new_v + v2t_new_v;
        }

         */
    }

    pub fn get_collision_axis_and_overlap(&self, other: &PhysicsObject) -> Option<(Vector2, f32)> {
        let mut u_axes_to_be_checked: Vec<Vector2> = Vec::new();
        u_axes_to_be_checked.extend(self.get_all_u_axes());
        u_axes_to_be_checked.extend(other.get_all_u_axes());

        let mut smallest_overlap: f32 = f32::INFINITY;
        let mut best_u_axis: Vector2 = Vector2::zero();

        for u_axis in u_axes_to_be_checked {
            let mut self_min: f32 = f32::INFINITY;
            let mut self_max: f32 = f32::NEG_INFINITY;
            let mut other_min: f32 = f32::INFINITY;
            let mut other_max: f32 = f32::NEG_INFINITY;

            let self_corners: &Vec<Vector2> = self.physics.get_corners();
            let other_corners: &Vec<Vector2> = other.physics.get_corners();
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
            if self_max <= other_min || other_max <= self_min {
                return None;
            }

            let overlap: f32 = f32::min(self_max, other_max) - f32::max(self_min, other_min);
            if overlap < smallest_overlap {
                smallest_overlap = overlap;
                best_u_axis = u_axis;
            }
        }
        Some((best_u_axis, smallest_overlap))
    }
    pub fn get_all_u_axes(&self) -> Vec<Vector2> {
        let mut result: Vec<Vector2> = Vec::new();
        let corners: &Vec<Vector2> = self.physics.get_corners();
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
        for corner in self.physics.get_corners() {
            let cell_coord_x: usize = (corner.x / WIDTH_F * cell_count_x as f32) as usize;
            let cell_coord_y: usize = (corner.y / HEIGHT_F * cell_count_y as f32) as usize;
            cells_put_into.insert((cell_coord_x, cell_coord_y));
        }

        cells_put_into
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
                for corner in self.physics.get_corners_mut() {
                    let new_d_vector: Vector2 = (*corner - self.obj.pos).rotated(added_rotation);
                    *corner = self.obj.pos + new_d_vector;
                }
            }
            Static { .. } => {
                // dont do anything
            }
        }
    }

    pub fn move_relative(&mut self, added_pos: &Vector2) {
        self.obj.pos += *added_pos;
        for corner in self.physics.get_corners_mut() {
            *corner += *added_pos;
        }
    }
    pub fn render(&self, d: &mut RaylibDrawHandle) {
        let corner_count: usize = self.physics.get_corners().len();
        for i in 0..corner_count {
            let first_corner: &Vector2 = &self.physics.get_corners()[i];
            let second_corner: &Vector2 = &self.physics.get_corners()[(i + 1) % corner_count];
            d.draw_line_ex(first_corner, second_corner, 5., self.obj.color);
        }
    }
}
