use crate::cell::GridCell;
use crate::grid::Grid;
use rand;
pub struct AldousBroder {}

impl AldousBroder {
    pub fn on(mut grid: Grid) -> Grid {
        let mut cell_ref = grid.get_random_cell().upgrade().unwrap();
        let mut unvisited = grid.size() - 1;
        while unvisited > 0 {
            let mut neighbours = cell_ref.borrow().neighbours();
            let rand_num = rand::random::<usize>() % neighbours.len();
            let mut rand_neighbour_ref = neighbours.remove(rand_num).upgrade().unwrap();
            if rand_neighbour_ref.borrow().get_links().is_empty() {
                GridCell::link(
                    &mut cell_ref.borrow_mut(),
                    &mut rand_neighbour_ref.borrow_mut(),
                );
                unvisited -= 1;
            }
            cell_ref = rand_neighbour_ref;
        }
        grid
    }
}
