use crate::rust_game_engine::constants::{HEIGHT_F, WIDTH_F};
use crate::rust_game_engine::engine_core::Scene;
use crate::rust_game_engine::physics::game_object::GameObject;
use std::collections::{HashMap, HashSet};

impl Scene {
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
        possible_collisions: &Vec<(usize, usize)>,
    ) -> Vec<(usize, usize)> {
        // collision detection - narrow phase
        let mut real_collision_pairs: Vec<(usize, usize)> = Vec::new();
        for &(i, j) in possible_collisions {
            if self.game_objects[i].collides_with(&self.game_objects[j]) {
                real_collision_pairs.push((i, j));
            }
        }
        real_collision_pairs
    }

    pub fn resolve_collisions(&mut self, collisions: &Vec<(usize, usize)>) {
        for &(i, j) in collisions {
            let (left, right) = self.game_objects.split_at_mut(j);
            left[i].resolve_collision_other(&mut right[0]);
        }

        let mut object_set: HashSet<usize> = HashSet::new();
        for &(i, j) in collisions {
            object_set.insert(i);
            object_set.insert(j);
        }
        for i in object_set {
            self.game_objects[i].resolve_collision_walls();
        }
    }
}
