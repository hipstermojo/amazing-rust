use std::rc::Rc;

use rand;

use crate::distances::Distances;
use crate::{cell, cell::Coord};

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

    pub fn reset(&mut self) {
        for row in 0..self.rows {
            for col in 0..self.columns {
                let cell = &self.grid[row][col];
                cell.borrow_mut().reset();
            }
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

    pub fn distances(&self) -> Distances {
        let mut distances = Distances::initialize(Coord::from(0, 0));
        let mut frontier = vec![self.get_cell_ref(0, 0).unwrap()];
        while !frontier.is_empty() {
            let mut new_frontier = Vec::new();
            for weak_cell_ref in &mut frontier {
                let strong_cell_ref = weak_cell_ref.upgrade().unwrap();
                let current_distance = distances.get_cell_distance(&Coord::from(
                    strong_cell_ref.borrow().row,
                    strong_cell_ref.borrow().column,
                ));
                strong_cell_ref
                    .borrow()
                    .links
                    .iter()
                    .filter(|coord| !distances.has_cell(coord))
                    .cloned()
                    .for_each(|unvisited| {
                        new_frontier.push(
                            self.get_cell_ref(unvisited.row(), unvisited.column())
                                .unwrap(),
                        )
                    });
                for weak_ref in &new_frontier {
                    let strong_ref = weak_ref.upgrade().unwrap();
                    distances.set_cell_distance(
                        Coord::from(strong_ref.borrow().row, strong_ref.borrow().column),
                        current_distance + 1,
                    );
                }
            }
            frontier = new_frontier;
        }
        distances
    }

    pub fn contents_of(&self, cell: &cell::GridCell) -> String {
        let distances = self.distances();
        let cell_distance = distances
            .cells
            .get(&Coord::from(cell.row, cell.column))
            .map(|x| std::char::from_digit(*x as u32, 32).unwrap())
            .unwrap_or(' ');
        format!(" {} ", cell_distance.to_string())
    }
}

impl ToString for Grid {
    fn to_string(&self) -> String {
        let mut output = String::new();
        let section = "+".to_owned() + &"---+".repeat(self.columns) + "\n";
        output.push_str(&section);
        self.grid
            .iter()
            .map(|row| {
                let top = "|".to_owned();
                let bottom = "+".to_owned();
                let (top, bottom) = row.iter().fold((top, bottom), |acc, cell| {
                    let body = self.contents_of(&cell.borrow());
                    let east_boundary = if let Some(eastern_neighbour) = &cell.borrow().east {
                        let eastern_cell = eastern_neighbour.upgrade().unwrap();
                        let east_coords =
                            Coord::from(eastern_cell.borrow().row, eastern_cell.borrow().column);
                        if cell.borrow().is_linked(&east_coords) {
                            " "
                        } else {
                            "|"
                        }
                    } else {
                        "|"
                    };
                    let south_boundary = if let Some(southern_neighbour) = &cell.borrow().south {
                        let southern_cell = southern_neighbour.upgrade().unwrap();
                        let south_coords =
                            Coord::from(southern_cell.borrow().row, southern_cell.borrow().column);
                        if cell.borrow().is_linked(&south_coords) {
                            "   "
                        } else {
                            "---"
                        }
                    } else {
                        "---"
                    };
                    (acc.0 + &body + east_boundary, acc.1 + south_boundary + "+")
                });
                (top, bottom)
            })
            .for_each(|(top, bottom)| {
                output.push_str(&top);
                output.push_str("\n");
                output.push_str(&bottom);
                output.push_str("\n");
            });
        output
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
