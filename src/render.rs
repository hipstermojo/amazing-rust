use cairo::{Context, Format, ImageSurface};
use std::fs::File;

use crate::cell::{Coord, GridCellRef};
use crate::grid::Grid;

enum Direction {
    Horizontal,
    Vertical,
}

pub struct Dimension {
    width: f64,
    height: f64,
}

#[allow(unused_variables)]
pub trait Renderable {
    fn to_png(&self, filename: &str) {}
    fn draw_square_cell(
        &self,
        context: &Context,
        cell: &GridCellRef,
        x_index: usize,
        y_index: usize,
        padding: f64,
        cell_size: Dimension,
    ) {
    }
}

impl Renderable for Grid {
    fn to_png(&self, filename: &str) {
        const PADDING: f64 = 10.0;
        let image_width = (self.columns * 30) as i32 + (2 * PADDING as i32);
        let image_height = (self.rows * 30) as i32 + (2 * PADDING as i32);
        let surface = ImageSurface::create(Format::ARgb32, image_width, image_height)
            .expect("Could not generate ImageSurface");
        let context = Context::new(&surface);
        context.set_source_rgb(1.0, 1.0, 1.0);
        context.paint();

        context.set_source_rgb(0.0, 0.0, 0.0);

        // Draw northern border
        context.line_to(PADDING, PADDING);
        context.line_to(image_width as f64 - PADDING, PADDING);
        context.stroke();

        for row in 0..self.rows {
            // Draw western border
            context.line_to(PADDING as f64, PADDING + (30 * row) as f64);
            context.line_to(PADDING as f64, PADDING + (30 * (row + 1)) as f64);
            context.stroke();

            for col in 0..self.columns {
                let cell = &self.grid[row][col];
                let mut box_dimension = Dimension {
                    width: 28.0,
                    height: 28.0,
                };
                if let Some(eastern_neighbour) = &cell.borrow().east {
                    let eastern_ref = eastern_neighbour.upgrade().unwrap();
                    let eastern_coords =
                        Coord::from(eastern_ref.borrow().row, eastern_ref.borrow().column);
                    if cell.borrow().is_linked(&eastern_coords) {
                        box_dimension.width = 30.0;
                    } else {
                        draw_line(Direction::Vertical, &context, row, col + 1, PADDING);
                    }
                } else {
                    draw_line(Direction::Vertical, &context, row, col + 1, PADDING);
                }
                if let Some(southern_neighbour) = &cell.borrow().south {
                    let southern_ref = southern_neighbour.upgrade().unwrap();
                    let southern_coords =
                        Coord::from(southern_ref.borrow().row, southern_ref.borrow().column);
                    if cell.borrow().is_linked(&southern_coords) {
                        box_dimension.height = 30.0;
                    } else {
                        draw_line(Direction::Horizontal, &context, row + 1, col, PADDING);
                    }
                } else {
                    draw_line(Direction::Horizontal, &context, row + 1, col, PADDING);
                }
                self.draw_square_cell(&context, cell, col, row, PADDING, box_dimension);
            }
        }
        let mut file = File::create(filename).expect("Couldn't create an output file");
        surface
            .write_to_png(&mut file)
            .expect("Couldn't write to output file");
    }
    fn draw_square_cell(
        &self,
        context: &Context,
        cell: &GridCellRef,
        x_index: usize,
        y_index: usize,
        padding: f64,
        cell_size: Dimension,
    ) {
        let (_, max_distance) = self.distances.max();
        let intensity;
        if self
            .distances
            .has_cell(&Coord::from(cell.borrow().row, cell.borrow().column))
        {
            let distance = self
                .distances
                .get_cell_distance(&Coord::from(cell.borrow().row, cell.borrow().column));
            intensity = (max_distance - distance) as f64 / max_distance as f64;
        } else {
            intensity = 0.0;
        }
        context.rectangle(
            padding + 1.0 + (30 * x_index) as f64,
            padding + 1.0 + (30 * y_index) as f64,
            cell_size.width,
            cell_size.height,
        );
        let bright = 0.5 + (0.4 * intensity);
        context.set_source_rgb(intensity, bright, intensity);
        context.fill();
        context.set_source_rgb(0.0, 0.0, 0.0);
    }
}

fn draw_line(direction: Direction, context: &Context, row: usize, col: usize, padding: f64) {
    context.line_to(padding + (30 * col) as f64, padding + (30 * row) as f64);
    match direction {
        Direction::Horizontal => context.line_to(
            padding + (30 * (col + 1)) as f64,
            padding + (30 * row) as f64,
        ),
        Direction::Vertical => context.line_to(
            padding + (30 * col) as f64,
            padding + (30 * (row + 1)) as f64,
        ),
    }
    context.stroke();
}
