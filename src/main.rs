mod binary_tree;
mod cell;
mod grid;
mod side_winder;

use grid::Grid;

fn main() {
    println!("I am making some amazing things");
    let mut grid = Grid::initialize(4, 4);
    grid.configure_cells();
    {
        let binary_tree_grid = binary_tree::BinaryTree::on(grid.clone());
        println!(
            "Here's the maze using the binary tree algorithm!\n{}",
            binary_tree_grid.to_string()
        );
    }
    grid.reset();
    {
        let side_winder_grid = side_winder::SideWinder::on(grid.clone());
        println!(
            "Here's the maze using the Sidewinder algorithm!\n{}",
            side_winder_grid.to_string()
        );
    }
}
