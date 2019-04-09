use std::mem;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

#[derive(Clone, Copy)]
enum Cell {
    Dead,
    Alive,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    canvas: Vec<u8>,
    cells: Vec<Cell>,
    cells_tmp: Vec<Cell>,
}

/// Private methods not exposed to JavaScript.
impl Universe {
    #[inline]
    fn cell_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    #[inline]
    fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        for &delta_row in [self.height - 1, 0, 1].iter() {
            for &delta_col in [self.width - 1, 0, 1].iter() {
                // Skip center point.
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (col + delta_col) % self.width;
                let index = self.cell_index(neighbor_row, neighbor_col);
                // if let Cell::Alive = self.cells[index] {
                if let Cell::Alive = unsafe { self.cells.get_unchecked(index) } {
                    count += 1;
                }
            }
        }
        count
    }

    // Canvas methods.

    #[inline]
    fn make_dead(&mut self, index: usize) {
        *unsafe { self.canvas.get_unchecked_mut(index) } = 0;
        *unsafe { self.canvas.get_unchecked_mut(index + 1) } = 0;
        *unsafe { self.canvas.get_unchecked_mut(index + 2) } = 0;
        *unsafe { self.canvas.get_unchecked_mut(index + 3) } = 255;
    }

    #[inline]
    fn make_alive(&mut self, index: usize) {
        *unsafe { self.canvas.get_unchecked_mut(index) } = 255;
        *unsafe { self.canvas.get_unchecked_mut(index + 1) } = 255;
        *unsafe { self.canvas.get_unchecked_mut(index + 2) } = 255;
        *unsafe { self.canvas.get_unchecked_mut(index + 3) } = 255;
    }

    fn update_canvas_from_cells(&mut self) {
        let size = self.cells.len();
        for index in 0..size {
            let canvas_index = 4 * index;
            match self.cells[index] {
                Cell::Alive => self.make_alive(canvas_index),
                Cell::Dead => self.make_dead(canvas_index),
            }
        }
    }
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    pub fn canvas_data(&self) -> *const u8 {
        self.canvas.as_ptr()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn tick(&mut self, ctx: &CanvasRenderingContext2d) {
        // Update cells.
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.cell_index(row, col);
                let cell = unsafe { self.cells.get_unchecked(idx) };
                // let cell = &self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);
                let next_cell = match (cell, live_neighbors) {
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, _) => {
                        ctx.set_fill_style(&JsValue::from_str("#000000"));
                        ctx.fill_rect(col as f64, row as f64, 1.0, 1.0);
                        Cell::Dead
                    }
                    (Cell::Dead, 3) => {
                        ctx.set_fill_style(&JsValue::from_str("#FFFFFF"));
                        ctx.fill_rect(col as f64, row as f64, 1.0, 1.0);
                        Cell::Alive
                    }
                    _ => Cell::Dead,
                };
                *unsafe { self.cells_tmp.get_unchecked_mut(idx) } = next_cell;
            }
        }
        mem::swap(&mut self.cells, &mut self.cells_tmp);
    }

    pub fn new(width: u32, height: u32) -> Universe {
        let size = width * height;
        let cells = (0..size)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        let mut universe = Universe {
            width,
            height,
            canvas: vec![0; 4 * size as usize],
            cells,
            cells_tmp: vec![Cell::Dead; size as usize],
        };

        universe.update_canvas_from_cells();
        universe
    }

    pub fn draw(&mut self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        let image_data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&mut self.canvas),
            self.width,
            self.height,
        )?;
        ctx.put_image_data(&image_data, 0.0, 0.0)
    }
}
