use std::collections::HashSet;
use crate::heightmap::HeightMap;

#[derive(Debug)]
pub struct Bassin<'map> {
    to_visit: Vec<(usize, usize)>,
    terrain: HashSet<(usize, usize)>,
    heightmap: &'map HeightMap
}

impl<'map> Bassin<'map> {
    pub fn new(map: &'map HeightMap, low_point: (usize, usize)) -> Self {
        let mut terrain = HashSet::new();
        terrain.insert(low_point);

        Self {
            to_visit: vec![low_point],
            terrain,
            heightmap: map
        }
    }

    pub fn discover_terrain(&mut self) {
        while let Some((current_pt_x, current_pt_y)) = self.to_visit.pop() {
            // Explore the four cardinal directions of the point
            for (direction_x, direction_y) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let potentially_visitable = (current_pt_x as isize + direction_x, current_pt_y as isize + direction_y);
                let potentially_visitable_usize = (potentially_visitable.0 as usize, potentially_visitable.1 as usize);

                if self.to_visit.contains(&potentially_visitable_usize) || self.terrain.contains(&potentially_visitable_usize) {
                    continue;
                }

                let point_value = match self.heightmap.get_signed(potentially_visitable.0, potentially_visitable.1) {
                    Some(value) => value,
                    None => continue, // Map border
                };

                // If the height is 9, we are reaching the end of the bassin
                // A point with height 9 doesn't belong anywhere
                if point_value == 9 {
                    continue;
                }

                self.to_visit.push(potentially_visitable_usize);
                self.terrain.insert(potentially_visitable_usize);
            }
        }
    }

    pub fn terrain_size(&self) -> usize {
        self.terrain.len()
    }
}