use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::{Rc, Weak};

pub type GridCellRef = Rc<RefCell<GridCell>>;
pub type GridCellRefWeak = Weak<RefCell<GridCell>>;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Coord(usize, usize);

#[derive(Debug)]
pub struct GridCell {
    pub row: usize,
    pub column: usize,
    pub north: Option<GridCellRefWeak>,
    pub south: Option<GridCellRefWeak>,
    pub east: Option<GridCellRefWeak>,
    pub west: Option<GridCellRefWeak>,
    pub links: HashSet<Coord>,
}

impl GridCell {
    pub fn init(row: usize, column: usize) -> GridCellRef {
        Rc::new(RefCell::new(GridCell {
            row: row,
            column: column,
            north: None,
            south: None,
            east: None,
            west: None,
            links: HashSet::new(),
        }))
    }

    pub fn link(left: &mut GridCell, right: &mut GridCell) {
        left.links.insert(Coord(right.row, right.column));
        right.links.insert(Coord(left.row, left.column));
    }

    pub fn unlink(left: &mut GridCell, right: &mut GridCell) {
        let left_coord;
        let right_coord;
        {
            left_coord = Coord(left.row, left.column);
            right_coord = Coord(right.row, right.column);
        }
        left.links.remove(&right_coord);
        right.links.remove(&left_coord);
    }

    pub fn get_links(&self) -> Vec<Coord> {
        self.links.iter().map(|x| x.clone()).collect()
    }

    pub fn is_linked(&self, coord: &Coord) -> bool {
        self.links.contains(coord)
    }

    pub fn neighbours(&self) -> Vec<GridCellRefWeak> {
        let mut neighbours = Vec::new();
        if let Some(cell) = &self.north {
            neighbours.push(cell.clone());
        }
        if let Some(cell) = &self.east {
            neighbours.push(cell.clone());
        }
        if let Some(cell) = &self.west {
            neighbours.push(cell.clone());
        }
        if let Some(cell) = &self.south {
            neighbours.push(cell.clone());
        }
        neighbours
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn initializes_gridcell() {
        let grid_cell = GridCell::init(0, 0);
        assert_eq!(0, grid_cell.borrow().row);
        assert_eq!(0, grid_cell.borrow().column);
        assert!(grid_cell.borrow().north.is_none());
        assert!(grid_cell.borrow().east.is_none());
        assert!(grid_cell.borrow().west.is_none());
        assert!(grid_cell.borrow().south.is_none());
        assert_eq!(0, grid_cell.borrow().links.len());
    }

    #[test]
    fn links_two_cells() {
        let cell_a = GridCell::init(0, 0);
        let cell_b = GridCell::init(0, 1);
        GridCell::link(&mut cell_a.borrow_mut(), &mut cell_b.borrow_mut());
        assert!(cell_a.borrow().links.contains(&Coord(0, 1)));
        assert!(cell_b.borrow().links.contains(&Coord(0, 0)));
        let cell_c = GridCell::init(1, 0);
        GridCell::link(&mut cell_a.borrow_mut(), &mut cell_c.borrow_mut());
        assert!(cell_a.borrow().links.contains(&Coord(1, 0)));
    }

    #[test]
    fn unlinks_two_cells() {
        let cell_a = GridCell::init(0, 0);
        let cell_b = GridCell::init(0, 1);
        GridCell::link(&mut cell_a.borrow_mut(), &mut cell_b.borrow_mut());
        GridCell::unlink(&mut cell_a.borrow_mut(), &mut cell_b.borrow_mut());
        assert_eq!(0, cell_a.borrow().get_links().len());
        assert_eq!(0, cell_b.borrow().get_links().len());
    }

    #[test]
    fn returns_vec_of_links() {
        let cell_a = GridCell::init(0, 0);
        let cell_b = GridCell::init(0, 1);
        let cell_c = GridCell::init(1, 0);

        GridCell::link(&mut cell_a.borrow_mut(), &mut cell_b.borrow_mut());
        GridCell::link(&mut cell_a.borrow_mut(), &mut cell_c.borrow_mut());

        assert_eq!(2, cell_a.borrow().get_links().len());
    }

    #[test]
    fn verifies_is_linked() {
        let cell_a = GridCell::init(0, 0);
        let cell_b = GridCell::init(0, 1);
        let cell_c = GridCell::init(1, 0);

        GridCell::link(&mut cell_a.borrow_mut(), &mut cell_b.borrow_mut());
        GridCell::link(&mut cell_a.borrow_mut(), &mut cell_c.borrow_mut());

        assert!(cell_a.borrow().is_linked(&Coord(0, 1)));
        assert_eq!(false, cell_c.borrow().is_linked(&Coord(0, 1)));
    }

    #[test]
    fn returns_vec_of_neighbours() {
        let cell_a = GridCell::init(0, 0);
        let cell_b = GridCell::init(0, 1);

        assert_eq!(0, cell_a.borrow().neighbours().len());

        cell_a.borrow_mut().north = Some(Rc::downgrade(&cell_b));

        assert_eq!(1, cell_a.borrow().neighbours().len());
    }
}
