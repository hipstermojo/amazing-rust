use std::rc::Rc;

use crate::cell;
use rand;

type GridCells = Vec<Vec<cell::GridCellRef>>;

#[derive(Debug, Clone)]
pub struct Grid {
    pub rows: usize,
    pub columns: usize,
    pub grid: GridCells,
}

impl Grid {
    pub fn initialize(rows: usize, columns: usize) -> Grid {
        let grid = Grid {
            rows: rows,
            columns: columns,
            grid: Grid::prepare_grid(rows, columns),
        };
        grid
    }

    pub fn prepare_grid(rows: usize, columns: usize) -> GridCells {
        (0..rows)
            .map(|r| {
                (0..columns)
                    .map(|c| cell::GridCell::init(r, c))
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    pub fn configure_cells(&mut self) {
        for row in 0..self.rows {
            for column in 0..self.columns {
                let mut cell = self.grid[row][column].borrow_mut();
                let north = if row > 0 {
                    self.get_cell_ref(row - 1, column)
                } else {
                    None
                };
                let east = if column + 1 < self.columns {
                    self.get_cell_ref(row, column + 1)
                } else {
                    None
                };
                let west = if column > 0 {
                    self.get_cell_ref(row, column - 1)
                } else {
                    None
                };
                let south = if row + 1 < self.columns {
                    self.get_cell_ref(row + 1, column)
                } else {
                    None
                };
                cell.north = north;
                cell.east = east;
                cell.west = west;
                cell.south = south;
            }
        }
    }

    pub fn get_cell_ref(&self, row: usize, column: usize) -> Option<cell::GridCellRefWeak> {
        if let Some(row) = self.grid.get(row) {
            if let Some(cell) = row.get(column) {
                Some(Rc::downgrade(&cell.clone()))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_random_cell(&self) -> cell::GridCellRefWeak {
        let rand_row = rand::random::<usize>() % self.rows;
        let rand_col = rand::random::<usize>() % self.columns;
        self.get_cell_ref(rand_row, rand_col).unwrap()
    }

    pub fn size(&self) -> usize {
        self.rows * self.columns
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initializes_grid() {
        let new_grid = Grid::initialize(10, 5);
        assert_eq!(10, new_grid.rows);
        assert_eq!(5, new_grid.columns);
        assert_eq!(10, new_grid.grid.len());
        assert_eq!(5, new_grid.grid[0].len());
    }

    #[test]
    fn prepares_grid() {
        let grid_of_cells = Grid::prepare_grid(20, 7);
        assert_eq!(20, grid_of_cells.len());
        assert_eq!(7, grid_of_cells[0].len());
    }

    #[test]
    fn configures_cells() {
        let mut grid = Grid::initialize(10, 5);
        let cell_ref = grid.get_cell_ref(0, 0).unwrap().upgrade().unwrap();
        assert!(cell_ref.borrow().north.is_none());
        let cell_ref = grid.get_cell_ref(2, 2).unwrap().upgrade().unwrap();
        grid.configure_cells();

        // Assert that the neighbours exist
        assert!(cell_ref.borrow().north.is_some());
        assert!(cell_ref.borrow().east.is_some());
        assert!(cell_ref.borrow().west.is_some());
        assert!(cell_ref.borrow().south.is_some());

        // Assert that the neighbours are correct
        let northern_neighbour_ref = cell_ref.borrow().north.clone().unwrap().upgrade().unwrap();
        assert_eq!(
            (1, 2),
            (
                northern_neighbour_ref.borrow().row,
                northern_neighbour_ref.borrow().column
            )
        );
        let eastern_neighbour_ref = cell_ref.borrow().east.clone().unwrap().upgrade().unwrap();
        assert_eq!(
            (2, 3),
            (
                eastern_neighbour_ref.borrow().row,
                eastern_neighbour_ref.borrow().column
            )
        );
        let western_neighbour_ref = cell_ref.borrow().west.clone().unwrap().upgrade().unwrap();
        assert_eq!(
            (2, 1),
            (
                western_neighbour_ref.borrow().row,
                western_neighbour_ref.borrow().column
            )
        );
        let southern_neighbour_ref = cell_ref.borrow().south.clone().unwrap().upgrade().unwrap();
        assert_eq!(
            (3, 2),
            (
                southern_neighbour_ref.borrow().row,
                southern_neighbour_ref.borrow().column
            )
        );
    }

    #[test]
    fn gets_correct_cell_ref() {
        let mut grid = Grid::initialize(2, 4);
        grid.configure_cells();
        assert!(grid.get_cell_ref(20, 45).is_none());
        let cell_ref = grid.get_cell_ref(1, 3).unwrap().upgrade().unwrap();
        assert_eq!((1, 3), (cell_ref.borrow().row, cell_ref.borrow().column));
    }
}
