use cairo::{Context, Format, ImageSurface};
use std::fs::File;

use crate::cell::Coord;
use crate::grid::Grid;

pub trait Renderable {
    fn to_png(&self) {}
}

impl Renderable for Grid {
    fn to_png(&self) {
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
            let image_width = image_width as f64;

            // Draw western border
            context.line_to(PADDING as f64, PADDING + (30 * row) as f64);
            context.line_to(PADDING as f64, PADDING + (30 * (row + 1)) as f64);
            context.stroke();

            for col in 0..self.columns {
                let cell = &self.grid[row][col];
                if let Some(eastern_neighbour) = &cell.borrow().east {
                    let eastern_ref = eastern_neighbour.upgrade().unwrap();
                    let eastern_coords =
                        Coord::from(eastern_ref.borrow().row, eastern_ref.borrow().column);
                    if cell.borrow().is_linked(&eastern_coords) == false {
                        draw_vertical(&context, row, col + 1, PADDING);
                    }
                } else {
                    draw_vertical(&context, row, col + 1, PADDING);
                }
                if let Some(southern_neighbour) = &cell.borrow().south {
                    let southern_ref = southern_neighbour.upgrade().unwrap();
                    let southern_coords =
                        Coord::from(southern_ref.borrow().row, southern_ref.borrow().column);
                    if cell.borrow().is_linked(&southern_coords) == false {
                        draw_horizontal(&context, row + 1, col, PADDING);
                    }
                } else {
                    draw_horizontal(&context, row + 1, col, PADDING);
                }
            }

            // Draw eastern border
            context.line_to(image_width - PADDING, PADDING + (30 * row) as f64);
            context.line_to(image_width - PADDING, PADDING + (30 * (row + 1)) as f64);
            context.stroke();
        }

        // Draw southern border
        context.line_to(PADDING, image_height as f64 - PADDING);
        context.line_to(image_width as f64 - PADDING, image_height as f64 - PADDING);
        context.stroke();

        let mut file = File::create("output.png").expect("Couldn't create an output file");
        surface
            .write_to_png(&mut file)
            .expect("Couldn't write to output file");
    }
}

fn draw_horizontal(context: &Context, row: usize, col: usize, padding: f64) {
    context.line_to(padding + (30 * col) as f64, padding + (30 * row) as f64);
    context.line_to(
        padding + (30 * (col + 1)) as f64,
        padding + (30 * row) as f64,
    );
    context.stroke();
}

fn draw_vertical(context: &Context, row: usize, col: usize, padding: f64) {
    context.line_to(padding + (30 * col) as f64, padding + (30 * row) as f64);
    context.line_to(
        padding + (30 * col) as f64,
        padding + (30 * (row + 1)) as f64,
    );
    context.stroke();
}
