use std::collections::HashSet;
use crate::rust_game_engine::constants::*;
use crate::rust_game_engine::engine_core::*;
use raylib::prelude::*;

pub enum PhysicsObjectType {
    BALL{
        obj: GameObject,
        physics: PhysicsAddition,
        radius: f32,
    },
    SQUARE{
        obj: GameObject,
        physics: PhysicsAddition,
        side_length: f32,
    },
}
pub struct GameObject {
    pub pos: Vector2,
}
pub struct PhysicsAddition {
    pub vel: Vector2,
    pub accel: Vector2,
    pub mass: f32,
}

impl PhysicsObjectType {
    pub fn new_ball(pos: Vector2, mass: f32) -> PhysicsObjectType {
        let radius: f32 = (mass / PI as f32).sqrt();
        PhysicsObjectType::BALL {
            obj: GameObject {
                pos,
            },
            physics : PhysicsAddition{
                vel: Vector2::zero(),
                accel: Vector2::new(0.0, GRAVITY),
                mass,
            },
            radius,
        }
    }


    pub fn new_square(pos: Vector2, mass: f32) -> PhysicsObjectType {
        let side_length: f32 = mass.sqrt();
        PhysicsObjectType::SQUARE {
            obj: GameObject {
                pos,
            },
            physics : PhysicsAddition{
                vel: Vector2::zero(),
                accel: Vector2::new(0.0, GRAVITY),
                mass,
            },
            side_length
        }
    }

    pub fn resolve_collision_other(&mut self, other: &mut PhysicsObjectType) {
        match (self, other) {
            (PhysicsObjectType::BALL {
                obj: self_obj,
                physics: self_physics,
                radius: self_radius,
            },
            PhysicsObjectType::BALL {
                obj: other_obj,
                physics: other_physics,
                radius: other_radius,
            }) => {
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
                let buffer: f32 = 0.0;
                let overlap: f32 = *self_radius + *other_radius - dist + buffer;

                // bias
                let travel_percentage: f32 = 0.8;
                let travel_dist_self: f32 = overlap * m1 / (m1 + m2) * travel_percentage;
                let travel_dist_other: f32 = overlap * m2 / (m1 + m2) * travel_percentage;

                let dir_self_other: Vector2 = (self_obj.pos - other_obj.pos).normalized(); // some buffer space

                let correction_self: Vector2 = dir_self_other.scale_by(travel_dist_self);
                let correction_other: Vector2 = -dir_self_other.scale_by(travel_dist_other);

                // separate objects
                self_obj.pos += correction_self;
                other_obj.pos += correction_other;
            }

            (_, _) => {
                todo!()
            }
        }
    }


    pub fn resolve_collision_walls(&mut self) {
        match self {
            PhysicsObjectType::BALL{
                obj,
                physics,
                radius,
            } => {
                let left_x: f32 = obj.pos.x - *radius;
                let right_x: f32 = obj.pos.x + *radius;
                let upper_y: f32 = obj.pos.y - *radius;
                let down_y: f32 = obj.pos.y + *radius;

                clamp(&mut obj.pos.x, *radius, WIDTH_F - *radius);
                clamp(&mut obj.pos.y, *radius, HEIGHT_F - *radius);

                if left_x < 0. || right_x > WIDTH_F {
                    physics.vel.x *= -BOUNCINESS;
                }
                if upper_y < 0. || down_y > HEIGHT_F {
                    physics.vel.y *= -BOUNCINESS;
                }

            }
            _ => {
                todo!()
            }
        }
    }

    pub fn collides_with(&self, other: &PhysicsObjectType) -> bool {
        match (self, other) {
            (PhysicsObjectType::BALL{
                obj: self_obj,
                radius: self_radius,
                ..
            },
                PhysicsObjectType::BALL{
                    obj: other_obj,
                    radius: other_radius,
                    ..
                }) => {
                let dist_squared: f32 = (self_obj.pos - other_obj.pos).length_sqr();
                let radius_sum_squared: f32 = (self_radius + other_radius).powi(2);
                dist_squared < radius_sum_squared
            }

            _ => {
                todo!()
            }
        }
    }

    pub fn get_cell_positions(&self, cell_count_x: usize, cell_count_y: usize) -> HashSet<(usize, usize)> {
        match self {
            PhysicsObjectType::BALL{obj, radius, ..} => {
                let mut cells_put_into: HashSet<(usize, usize)> = HashSet::new();

                let offsets: [(f32, f32); 4] = [(-1., -1.), (-1., 1.), (1., -1.), (1., 1.)];
                for (dx, dy) in offsets {
                    let x: f32 = obj.pos.x + radius * dx;
                    let y: f32 = obj.pos.y + radius * dy;
                    let cell_coord_x: usize = (x / WIDTH_F * cell_count_x as f32) as usize;
                    let cell_coord_y: usize = (y / HEIGHT_F * cell_count_y as f32) as usize;
                    cells_put_into.insert((cell_coord_x, cell_coord_y));
                }
                cells_put_into
            }
            _ => {
                todo!()
            }
        }

    }

    pub fn update_move(&mut self, delta_time: f32) {
        let added_vel: Vector2 = self.get_physics_obj().accel * delta_time;
        self.get_physics_obj_mut().vel += added_vel;

        let added_pos: Vector2 = self.get_physics_obj().vel * delta_time;
        self.get_game_obj_mut().pos += added_pos;

        self.resolve_collision_walls();
    }
    pub fn render(&self, d: &mut RaylibDrawHandle) {
        match self {
            PhysicsObjectType::BALL{obj, physics, radius} => {
                d.draw_circle_v(obj.pos, *radius, Color::BLACK);
            }
            PhysicsObjectType::SQUARE{obj, physics, side_length} => {
                d.draw_rectangle_v(obj.pos, Vector2::new(*side_length, *side_length), Color::BLACK);
            }
        }
    }

    pub fn get_game_obj(&self) -> &GameObject {
        match self {
            PhysicsObjectType::BALL{obj, ..} => obj,
            PhysicsObjectType::SQUARE{obj, ..} => obj,
        }
    }
    pub fn get_game_obj_mut(&mut self) -> &mut GameObject {
        match self {
            PhysicsObjectType::BALL{obj, ..} => obj,
            PhysicsObjectType::SQUARE{obj, ..} => obj,
        }
    }
    pub fn get_physics_obj(&self) -> &PhysicsAddition {
        match self {
            PhysicsObjectType::BALL{physics, ..} => physics,
            PhysicsObjectType::SQUARE {physics, ..} => physics,
        }
    }
    pub fn get_physics_obj_mut(&mut self) -> &mut PhysicsAddition {
        match self {
            PhysicsObjectType::BALL{physics, ..} => physics,
            PhysicsObjectType::SQUARE {physics, ..} => physics,
        }
    }
}