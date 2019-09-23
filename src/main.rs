mod aldous_broder;
mod binary_tree;
mod cell;
mod distances;
mod grid;
mod render;
mod side_winder;

use cell::Coord;
use grid::Grid;
use render::Renderable;

fn main() {
    println!("I am making some amazing things");
    let mut grid = Grid::initialize(25, 25);
    grid.configure_cells();
    {
        let mut binary_tree_grid = binary_tree::BinaryTree::on(grid.clone());
        println!(
            "Here's the maze using the binary tree algorithm!\n{}",
            binary_tree_grid.to_string()
        );
        binary_tree_grid.distances = binary_tree_grid.find_distances(Coord::from(0, 0));
        let original_distances = binary_tree_grid.distances.clone();
        binary_tree_grid.distances =
            binary_tree_grid.path_to(Coord::from(binary_tree_grid.rows - 1, 0));
        println!(
            "Here's the path from NW to SW\n{}",
            binary_tree_grid.to_string()
        );
        binary_tree_grid.distances = original_distances;
        binary_tree_grid.distances = binary_tree_grid.longest_path();
        println!(
            "Here's the most difficult path in the maze\n{}",
            binary_tree_grid.to_string()
        );
        binary_tree_grid.to_png("binary_tree.png");
    }
    grid.reset();
    {
        let mut side_winder_grid = side_winder::SideWinder::on(grid.clone());
        println!(
            "Here's the maze using the Sidewinder algorithm!\n{}",
            side_winder_grid.to_string()
        );
        side_winder_grid.distances = side_winder_grid.find_distances(Coord::from(0, 0));
        let original_distances = side_winder_grid.distances.clone();
        side_winder_grid.distances =
            side_winder_grid.path_to(Coord::from(side_winder_grid.rows - 1, 0));
        println!(
            "Here's the path from NW to SW\n{}",
            side_winder_grid.to_string()
        );
        side_winder_grid.distances = original_distances;
        side_winder_grid.distances = side_winder_grid.longest_path();
        println!(
            "Here's the most difficult path in the maze\n{}",
            side_winder_grid.to_string()
        );
        side_winder_grid.to_png("sidewinder.png");
    }
    grid.reset();
    {
        let mut aldous_broder_grid = aldous_broder::AldousBroder::on(grid.clone());
        println!(
            "Here's the maze using the Aldous-Broder algorithm!\n{}",
            aldous_broder_grid.to_string()
        );
        aldous_broder_grid.distances = aldous_broder_grid.find_distances(Coord::from(0, 0));
        let original_distances = aldous_broder_grid.distances.clone();
        aldous_broder_grid.distances =
            aldous_broder_grid.path_to(Coord::from(aldous_broder_grid.rows - 1, 0));
        println!(
            "Here's the path from NW to SW\n{}",
            aldous_broder_grid.to_string()
        );
        aldous_broder_grid.distances = original_distances;
        aldous_broder_grid.distances = aldous_broder_grid.longest_path();
        println!(
            "Here's the most difficult path in the maze\n{}",
            aldous_broder_grid.to_string()
        );
        aldous_broder_grid.to_png("aldous_broder.png");
    }
}
