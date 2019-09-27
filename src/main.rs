mod aldous_broder;
mod binary_tree;
mod cell;
mod distances;
mod grid;
mod hunt_and_kill;
mod recursive_backtracker;
mod render;
mod side_winder;
mod wilsons;

use cell::Coord;
use grid::Grid;
use render::Renderable;

fn main() {
    println!("I am making some amazing things");
    let mut grid = Grid::initialize(20, 20);
    grid.configure_cells();
    let mut stats = Vec::new();
    {
        binary_tree::BinaryTree::on(&grid);
        stats.push(format!(
            "Binary Tree: {}/{} ({}%)",
            grid.deadends(),
            grid.size(),
            (grid.deadends() * 100) / grid.size()
        ));
        println!(
            "Here's the maze using the binary tree algorithm!\n{}",
            grid.to_string()
        );
        grid.distances = grid.find_distances(Coord::from(0, 0));
        let original_distances = grid.distances.clone();
        grid.distances = grid.path_to(Coord::from(grid.rows - 1, 0));
        println!("Here's the path from NW to SW\n{}", grid.to_string());
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
        stats.push(format!(
            "SideWinder: {}/{} ({}%)",
            grid.deadends(),
            grid.size(),
            (grid.deadends() * 100) / grid.size()
        ));
        println!(
            "Here's the maze using the Sidewinder algorithm!\n{}",
            grid.to_string()
        );
        grid.distances = grid.find_distances(Coord::from(0, 0));
        let original_distances = grid.distances.clone();
        grid.distances = grid.path_to(Coord::from(grid.rows - 1, 0));
        println!("Here's the path from NW to SW\n{}", grid.to_string());
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
        stats.push(format!(
            "Aldous Broder: {}/{} ({}%)",
            grid.deadends(),
            grid.size(),
            (grid.deadends() * 100) / grid.size()
        ));
        println!(
            "Here's the maze using the Aldous-Broder algorithm!\n{}",
            grid.to_string()
        );
        grid.distances = grid.find_distances(Coord::from(0, 0));
        let original_distances = grid.distances.clone();
        grid.distances = grid.path_to(Coord::from(grid.rows - 1, 0));
        println!("Here's the path from NW to SW\n{}", grid.to_string());
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
        stats.push(format!(
            "Wilson's: {}/{} ({}%)",
            grid.deadends(),
            grid.size(),
            (grid.deadends() * 100) / grid.size()
        ));
        println!(
            "Here's the maze using Wilson's algorithm!\n{}",
            grid.to_string()
        );
        grid.distances = grid.find_distances(Coord::from(0, 0));
        let original_distances = grid.distances.clone();
        grid.distances = grid.path_to(Coord::from(grid.rows - 1, 0));
        println!("Here's the path from NW to SW\n{}", grid.to_string());
        grid.distances = original_distances;
        grid.distances = grid.longest_path();
        println!(
            "Here's the most difficult path in the maze\n{}",
            grid.to_string()
        );
        grid.to_png("wilsons.png");
    }
    grid.reset();
    {
        hunt_and_kill::HuntAndKill::on(&grid);
        stats.push(format!(
            "Hunt and Kill: {}/{} ({}%)",
            grid.deadends(),
            grid.size(),
            (grid.deadends() * 100) / grid.size()
        ));
        println!(
            "Here's the maze using the Hunt and Kill algorithm!\n{}",
            grid.to_string()
        );
        grid.distances = grid.find_distances(Coord::from(0, 0));
        let original_distances = grid.distances.clone();
        grid.distances = grid.path_to(Coord::from(grid.rows - 1, 0));
        println!("Here's the path from NW to SW\n{}", grid.to_string());
        grid.distances = original_distances;
        grid.distances = grid.longest_path();
        println!(
            "Here's the most difficult path in the maze\n{}",
            grid.to_string()
        );
        grid.to_png("hunt_and_kill.png");
    }
    grid.reset();
    {
        recursive_backtracker::RecursiveBacktracker::on(&grid);
        stats.push(format!(
            "Recursive Backtracker: {}/{} ({}%)",
            grid.deadends(),
            grid.size(),
            (grid.deadends() * 100) / grid.size()
        ));
        println!(
            "Here's the maze using the Recursive Backtracker algorithm!\n{}",
            grid.to_string()
        );
        grid.distances = grid.find_distances(Coord::from(0, 0));
        let original_distances = grid.distances.clone();
        grid.distances = grid.path_to(Coord::from(grid.rows - 1, 0));
        println!("Here's the path from NW to SW\n{}", grid.to_string());
        grid.distances = original_distances;
        grid.distances = grid.longest_path();
        println!(
            "Here's the most difficult path in the maze\n{}",
            grid.to_string()
        );
        grid.to_png("recursive_backtracker.png");
    }
    println!(
        "Deadends of maze algorithms on a ({}x{}) maze",
        grid.rows, grid.columns
    );
    for stat in stats {
        println!("{}", stat);
    }
}
