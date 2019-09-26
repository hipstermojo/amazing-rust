use rand;

use crate::cell::GridCell;
use crate::grid::Grid;

pub struct BinaryTree {}

impl BinaryTree {
    pub fn on(grid: &Grid) {
        for row in 0..grid.rows {
            for col in 0..grid.rows {
                let mut neighbours = Vec::new();

                let cell = &grid.grid[row][col];
                if let Some(cell_ref) = &cell.borrow().north {
                    neighbours.push(cell_ref.clone());
                }
                if let Some(cell_ref) = &cell.borrow().east {
                    neighbours.push(cell_ref.clone());
                }
                match neighbours.len() {
                    0 => (),
                    1 => {
                        let neighbour_cell = neighbours.remove(0).upgrade().unwrap();
                        GridCell::link(&mut cell.borrow_mut(), &mut neighbour_cell.borrow_mut());
                    }
                    2 => {
                        let rand_index = rand::random::<usize>() % 2;
                        let neighbour_cell = neighbours.remove(rand_index).upgrade().unwrap();
                        neighbours.clear();
                        GridCell::link(&mut cell.borrow_mut(), &mut neighbour_cell.borrow_mut());
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}
