use std::rc::Rc;

use rand;

use crate::cell::{GridCell, GridCellRef};
use crate::grid::Grid;

pub struct RecursiveBacktracker {}

impl RecursiveBacktracker {
    pub fn on(grid: &Grid) {
        let start = grid.get_random_cell();
        let mut stack = Vec::new();
        stack.push(start);
        while !stack.is_empty() {
            let current = stack.last().unwrap().upgrade().unwrap();
            let unvisited_neighbours = current
                .borrow()
                .neighbours()
                .into_iter()
                .map(|weak_ref| weak_ref.upgrade().unwrap())
                .filter(|cell_ref| cell_ref.borrow().links.is_empty())
                .collect::<Vec<GridCellRef>>();
            if unvisited_neighbours.is_empty() {
                stack.pop();
            } else {
                let rand_num = rand::random::<usize>() % unvisited_neighbours.len();
                let neighbour = unvisited_neighbours.get(rand_num).unwrap();
                GridCell::link(&mut current.borrow_mut(), &mut neighbour.borrow_mut());
                stack.push(Rc::downgrade(neighbour));
            }
        }
    }
}
