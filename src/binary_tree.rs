use rand;
use std::rc::Rc;

use crate::cell::GridCell;
use crate::grid::Grid;

struct BinaryTree {}

impl BinaryTree {
    pub fn on(mut grid: Grid) -> Grid {
        grid.grid = (0..grid.rows)
            .map(|row| {
                (0..grid.columns)
                    .map(|col| {
                        let mut neighbours = Vec::new();

                        let mut cell = &grid.grid[row][col];
                        if let Some(cell_ref) = &cell.borrow().north {
                            neighbours.push(cell_ref.clone());
                        }
                        if let Some(cell_ref) = &cell.borrow().east {
                            neighbours.push(cell_ref.clone());
                        }
                        match neighbours.len() {
                            0 => cell.clone(),
                            1 => {
                                let neighbour_cell = neighbours.remove(0).upgrade().unwrap();
                                GridCell::link(
                                    &mut cell.borrow_mut(),
                                    &mut neighbour_cell.borrow_mut(),
                                );
                                cell.clone()
                            }
                            2 => {
                                let rand_index = rand::random::<usize>() % 2;
                                let neighbour_cell =
                                    neighbours.remove(rand_index).upgrade().unwrap();
                                neighbours.clear();
                                GridCell::link(
                                    &mut cell.borrow_mut(),
                                    &mut neighbour_cell.borrow_mut(),
                                );
                                cell.clone()
                            }
                            _ => unreachable!(),
                        }
                    })
                    .collect()
            })
            .collect();

        grid
    }
}
