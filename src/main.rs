mod aldous_broder;
mod binary_tree;
mod cell;
mod distances;
mod grid;
mod render;
mod side_winder;
mod wilsons;

use cell::Coord;
use grid::Grid;
use render::Renderable;

fn main() {
    println!("I am making some amazing things");
    let mut grid = Grid::initialize(25, 25);
    grid.configure_cells();
    {
        binary_tree::BinaryTree::on(&grid);
        println!(
            "Here's the maze using the binary tree algorithm!\n{}",
            grid.to_string()
        );
        grid.distances = grid.find_distances(Coord::from(0, 0));
        let original_distances = grid.distances.clone();
        grid.distances =
            grid.path_to(Coord::from(grid.rows - 1, 0));
        println!(
            "Here's the path from NW to SW\n{}",
            grid.to_string()
        );
        grid.distances = original_distances;
        grid.distances = grid.longest_path();
        println!(
            "Here's the most difficult path in the maze\n{}",
            grid.to_string()
        );
        grid.to_png("binary_tree.png");
    }
    grid.reset();
    {
        side_winder::SideWinder::on(&grid);
        println!(
            "Here's the maze using the Sidewinder algorithm!\n{}",
            grid.to_string()
        );
        grid.distances = grid.find_distances(Coord::from(0, 0));
        let original_distances = grid.distances.clone();
        grid.distances =
            grid.path_to(Coord::from(grid.rows - 1, 0));
        println!(
            "Here's the path from NW to SW\n{}",
            grid.to_string()
        );
        grid.distances = original_distances;
        grid.distances = grid.longest_path();
        println!(
            "Here's the most difficult path in the maze\n{}",
            grid.to_string()
        );
        grid.to_png("sidewinder.png");
    }
    grid.reset();
    {
        aldous_broder::AldousBroder::on(&grid);
        println!(
            "Here's the maze using the Aldous-Broder algorithm!\n{}",
            grid.to_string()
        );
        grid.distances = grid.find_distances(Coord::from(0, 0));
        let original_distances = grid.distances.clone();
        grid.distances =
            grid.path_to(Coord::from(grid.rows - 1, 0));
        println!(
            "Here's the path from NW to SW\n{}",
            grid.to_string()
        );
        grid.distances = original_distances;
        grid.distances = grid.longest_path();
        println!(
            "Here's the most difficult path in the maze\n{}",
            grid.to_string()
        );
        grid.to_png("aldous_broder.png");
    }
    grid.reset();
    {
        wilsons::Wilsons::on(&grid);
        println!(
            "Here's the maze using the Wilson's algorithm!\n{}",
            grid.to_string()
        );
        grid.distances = grid.find_distances(Coord::from(0, 0));
        let original_distances = grid.distances.clone();
        grid.distances = grid.path_to(Coord::from(grid.rows - 1, 0));
        println!(
            "Here's the path from NW to SW\n{}",
            grid.to_string()
        );
        grid.distances = original_distances;
        grid.distances = grid.longest_path();
        println!(
            "Here's the most difficult path in the maze\n{}",
            grid.to_string()
        );
        grid.to_png("wilsons.png");
    }
}
