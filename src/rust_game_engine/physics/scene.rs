use crate::rust_game_engine::engine_core::Scene;
use std::collections::{HashMap, HashSet};

impl Scene {
    pub fn get_possible_collisions(&self) -> HashSet<(usize, usize)> {
        // collision detection - broad phase
        let obj_count = self.game_objects.len();

        let mut cell_index_map: HashMap<(usize, usize), HashSet<usize>> = HashMap::new();
        // fill map
        for i in 0..obj_count {
            let cells_put_into: HashSet<(usize, usize)> =
                self.game_objects[i].get_cell_positions(self.space_partitioning_grid_size);

            for cell in cells_put_into {
                cell_index_map
                    .entry(cell)
                    .or_insert(HashSet::new())
                    .insert(i);
            }
        }

        let mut possible_collision_pairs: HashSet<(usize, usize)> = HashSet::new();

        for (_, objs_set) in cell_index_map {
            let objs_vec: Vec<usize> = Vec::from_iter(objs_set);

            for &real_idx_1 in &objs_vec {
                for &real_idx_2 in &objs_vec {
                    if real_idx_1 < real_idx_2 {
                        possible_collision_pairs.insert((real_idx_1, real_idx_2));
                    }
                }
            }
        }
        possible_collision_pairs
    }

    pub fn filter_real_collisions(
        &self,
        mut possible_collisions: HashSet<(usize, usize)>,
    ) -> Vec<(usize, usize)> {
        // collision detection - narrow phase
        possible_collisions.drain().filter(|&(i, j)| {
            self.game_objects[i].collides_with(&self.game_objects[j])
        }).collect()
    }

    pub fn resolve_collisions(&mut self, collisions: &Vec<(usize, usize)>) {
        for &(i, j) in collisions {
            let (left, right) = self.game_objects.split_at_mut(j);
            left[i].resolve_collision_other(&mut right[0]);
        }
    }
}
