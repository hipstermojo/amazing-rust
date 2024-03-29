use std::collections::HashMap;

use crate::cell::Coord;

#[derive(Debug, Clone)]
pub struct Distances {
    pub root: Coord,
    pub cells: HashMap<Coord, usize>,
}

impl Distances {
    pub fn initialize(root: Coord) -> Self {
        let mut distances = Distances {
            root: root.clone(),
            cells: HashMap::new(),
        };
        distances.cells.insert(root, 0);
        distances
    }

    pub fn get_cell_distance(&self, cell_coord: &Coord) -> usize {
        self.cells.get(cell_coord).cloned().unwrap()
    }

    pub fn has_cell(&self, cell_coord: &Coord) -> bool {
        self.cells.contains_key(cell_coord)
    }

    pub fn set_cell_distance(&mut self, cell_coord: Coord, distance: usize) {
        self.cells.insert(cell_coord, distance);
    }

    pub fn get_cells(&self) -> Vec<Coord> {
        self.cells.keys().cloned().collect()
    }

    pub fn max(&self) -> (Coord, usize) {
        let mut result = (self.root.clone(), 0);
        for (coord, distance) in self.cells.iter() {
            if distance > &result.1 {
                result = (coord.clone(), *distance);
            }
        }
        result
    }
}
