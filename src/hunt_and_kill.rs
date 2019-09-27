use rand;

use crate::cell::{GridCell, GridCellRef};
use crate::grid::Grid;

pub struct HuntAndKill {}

impl HuntAndKill {
    pub fn on(grid: &Grid) {
        let mut current = grid.get_random_cell().upgrade();
        'visiting: while current.is_some() {
            let unvisited_neighbours: Vec<GridCellRef> = current
                .as_ref()
                .unwrap()
                .borrow()
                .neighbours()
                .into_iter()
                .map(|weak_ref| weak_ref.upgrade().unwrap())
                .filter(|cell_ref| cell_ref.borrow().links.is_empty())
                .collect();
            if unvisited_neighbours.is_empty() {
                current = None;
                'hunting: for row in 0..grid.rows {
                    for col in 0..grid.columns {
                        let cell = &grid.grid[row][col];
                        let visited_neighbours: Vec<GridCellRef> = cell
                            .borrow()
                            .neighbours()
                            .into_iter()
                            .map(|weak_ref| weak_ref.upgrade().unwrap())
                            .filter(|cell_ref| !cell_ref.borrow().links.is_empty())
                            .collect();
                        if cell.borrow().links.is_empty() && !visited_neighbours.is_empty() {
                            let rand_num = rand::random::<usize>() % visited_neighbours.len();
                            let neighbour = visited_neighbours.get(rand_num).unwrap();
                            GridCell::link(&mut cell.borrow_mut(), &mut neighbour.borrow_mut());
                            current = Some(cell.clone());
                            break 'hunting;
                        }
                    }
                }
            } else {
                let rand_num = rand::random::<usize>() % unvisited_neighbours.len();
                let neighbour = unvisited_neighbours.get(rand_num).unwrap();
                GridCell::link(
                    &mut current.unwrap().borrow_mut(),
                    &mut neighbour.borrow_mut(),
                );
                current = Some(neighbour.clone());
            }
        }
    }
}
