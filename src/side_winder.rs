use std::cell::RefCell;
use std::rc::Rc;

use crate::cell::GridCell;
use crate::grid::Grid;
use rand;

pub struct SideWinder {}

impl SideWinder {
    pub fn on(grid: &Grid) {
        for row in 0..grid.rows {
            let mut run: Vec<Rc<RefCell<GridCell>>> = Vec::new();
            for col in 0..grid.columns {
                let cell = &grid.grid[row][col];
                run.push(cell.clone());
                let at_eastern_boundary = cell.borrow().east.is_none();
                let at_northern_boundary = cell.borrow().north.is_none();
                let should_close_out = at_eastern_boundary
                    || (!at_northern_boundary && rand::random::<usize>() % 2 == 0);
                if should_close_out {
                    let member = run.remove(rand::random::<usize>() % run.len());
                    if member.borrow().north.is_some() {
                        let northern_ref =
                            member.borrow().north.as_ref().unwrap().upgrade().unwrap();
                        GridCell::link(&mut member.borrow_mut(), &mut northern_ref.borrow_mut());
                        run.clear();
                    }
                } else {
                    let eastern_ref = cell.borrow().east.as_ref().unwrap().upgrade().unwrap();
                    GridCell::link(&mut cell.borrow_mut(), &mut eastern_ref.borrow_mut());
                }
            }
        }
    }
}
