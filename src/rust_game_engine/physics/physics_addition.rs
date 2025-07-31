use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::{Rectangle, Vector2};
use raylib::prelude::Color;
use std::f32::consts::{PI, TAU};

pub struct Polygon {
    pub corners: Vec<Vector2>,
    pub bounding_box: Rectangle,
}
pub enum PhysicsAddition {
    Dynamic {
        accel: Vector2,
        vel: Vector2,
        mass: f32,
        inv_mass: f32,
    },
    Static,
}

impl PhysicsAddition {
    pub fn get_masses(&self) -> (f32, f32) {
        if let PhysicsAddition::Dynamic { mass, inv_mass, .. } = self {
            (*mass, *inv_mass)
        } else {
            (f32::INFINITY, 0.)
        }
    }

    pub fn get_vel_mut(&mut self) -> Option<&mut Vector2> {
        if let PhysicsAddition::Dynamic { ref mut vel, .. } = self {
            Some(vel)
        } else {
            None
        }
    }

    pub fn is_dynamic(&self) -> bool {
        matches!(self, PhysicsAddition::Dynamic { .. })
    }
    pub fn is_static(&self) -> bool {
        matches!(self, PhysicsAddition::Static)
    }
}

impl Polygon {
    pub fn new_regular_polygon(pos: Vector2, corner_count: usize, radius: f32) -> Polygon {
        let mut corners: Vec<Vector2> = Vec::new();
        for i in 0..corner_count {
            let angle: f32 = i as f32 / corner_count as f32 * TAU + 0.9 * PI;
            let vector_relative: Vector2 = Vector2::new(0., 1.).scale_by(radius).rotated(angle);
            corners.push(pos + vector_relative);
        }
        let mut result: Polygon = Polygon {
            corners,
            bounding_box: Rectangle::default(),
        };
        result.update_bounding_box();
        result
    }

    pub fn new_polygon_line(start: Vector2, end: Vector2, thickness: f32) -> Polygon {
        let u_normal: Vector2 = (end - start).normalized();
        let u_tangent: Vector2 = Vector2::new(-u_normal.y, u_normal.x);

        let mut corners: Vec<Vector2> = Vec::new();
        corners.push(end - u_tangent * thickness);
        corners.push(start - u_tangent * thickness);
        corners.push(start + u_tangent * thickness);
        corners.push(end + u_tangent * thickness);

        let mut result: Polygon = Polygon {
            corners,
            bounding_box: Rectangle::default(),
        };
        result.update_bounding_box();
        result
    }

    pub fn move_relative(&mut self, added_pos: &Vector2) {
        for corner in &mut self.corners {
            *corner += *added_pos;
        }
        self.update_bounding_box();
    }
    pub fn update_bounding_box(&mut self) {
        let mut min_x: f32 = f32::INFINITY;
        let mut max_x: f32 = f32::NEG_INFINITY;
        let mut min_y: f32 = f32::INFINITY;
        let mut max_y: f32 = f32::NEG_INFINITY;

        for &Vector2 { x, y } in &self.corners {
            min_x = min_x.min(x);
            max_x = max_x.max(x);
            min_y = min_y.min(y);
            max_y = max_y.max(y);
        }
        self.bounding_box = Rectangle {
            x: min_x,
            y: min_y,
            width: max_x - min_x,
            height: max_y - min_y,
        };
    }

    pub fn render(&self, d: &mut RaylibDrawHandle, color: Color) {
        // polygon
        for i in 0..self.corners.len() {
            let first_corner: &Vector2 = &self.corners[i];
            let second_corner: &Vector2 = &self.corners[(i + 1) % self.corners.len()];
            d.draw_line_ex(first_corner, second_corner, 5., color);
        }

        // bounding box
        /*
        d.draw_rectangle_lines_ex(
            self.bounding_box,
            2.,
            color
        );
         */
    }
}
