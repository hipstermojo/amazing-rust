use crate::cell::{Coord, GridCell};
use crate::grid::Grid;
use rand;

pub struct Wilsons {}

impl Wilsons {
    pub fn on(grid: &Grid) {
        let mut unvisited = Vec::new();
        for row in 0..grid.rows {
            for column in 0..grid.columns {
                unvisited.push(Coord::from(row, column));
            }
        }
        let first = rand::random::<usize>() % unvisited.len();
        unvisited.remove(first);
        while !unvisited.is_empty() {
            let rand_num = rand::random::<usize>() % unvisited.len();
            let mut rand_cell_coord = unvisited.get(rand_num).cloned().unwrap();
            let mut path = vec![rand_cell_coord.clone()];
            while unvisited.contains(&rand_cell_coord) {
                let cell_ref = grid
                    .get_cell_ref(rand_cell_coord.row(), rand_cell_coord.column())
                    .unwrap()
                    .upgrade()
                    .unwrap();
                let neighbours = cell_ref.borrow().neighbours();
                let rand_num = rand::random::<usize>() % neighbours.len();
                rand_cell_coord = neighbours
                    .get(rand_num)
                    .map(|weak_ref| {
                        let cell_rc = weak_ref.upgrade().unwrap();
                        let coord = Coord::from(cell_rc.borrow().row, cell_rc.borrow().column);
                        coord
                    })
                    .unwrap();
                match path.iter().position(|coord| *coord == rand_cell_coord) {
                    Some(position) => {
                        path.split_off(position + 1);
                    }
                    None => path.push(rand_cell_coord.clone()),
                }
            }
            for i in 0..(path.len() - 1) {
                let left_cell = grid
                    .get_cell_ref(path[i].row(), path[i].column())
                    .unwrap()
                    .upgrade()
                    .unwrap();
                let right_cell = grid
                    .get_cell_ref(path[i + 1].row(), path[i + 1].column())
                    .unwrap()
                    .upgrade()
                    .unwrap();
                GridCell::link(&mut left_cell.borrow_mut(), &mut right_cell.borrow_mut());
                let index = unvisited
                    .iter()
                    .position(|coord| coord == &path[i])
                    .unwrap();
                unvisited.remove(index);
            }
        }
    }
}
