mod binary_tree;
mod cell;
mod grid;
mod side_winder;

use grid::Grid;

fn main() {
    println!("I am making some amazing things");
    let mut grid = Grid::initialize(7, 10);
    grid.configure_cells();
    grid = binary_tree::BinaryTree::on(grid);
    println!("Here's the maze!\n{}", grid.to_string());
}
