#![feature(gen_blocks)]

mod lake;
use lake::Environment;
use lake::Lake;

const X: bool = true;
const O: bool = false;

use macroquad::prelude::*;

fn iter_grid(
    dim_x: usize,
    dim_y: usize,
    w_cell: f32,
    h_cell: f32,
) -> impl Iterator<Item = (usize, usize, f32, f32)> {
    gen move {
        for iy in 0..dim_y {
            for ix in 0..dim_x {
                let x = (ix as f32 * w_cell) as f32;
                let y = (iy as f32 * h_cell) as f32;
                yield (ix, iy, x, y)
            }
        }
    }
}

fn draw_grid(dim_x: usize, dim_y: usize, map: Vec<bool>) {
    let w_cell = screen_width() / dim_x as f32;
    let h_cell = screen_height() / dim_y as f32;

    let it = iter_grid(dim_x, dim_y, w_cell, h_cell);
    for (ix, iy, x, y) in it {
        let color = if map[ix + dim_x * iy] == true {
            GREEN
        } else {
            RED
        };

        draw_rectangle(x, y, w_cell, h_cell, color);
        draw_rectangle_lines(x, y, w_cell, h_cell, 2., BLACK);
    }
}

#[macroquad::main("MyGame")]
async fn main() {
    #[rustfmt::skip]
    let map: Vec<_> = [
        [O, O, O, O],
        [O, X, O, X], 
        [O, O, O, X], 
        [X, O, O, O]
        ]
        .iter()
        .flatten()
        .cloned()
        .collect();

    let mut lake = Lake::new(&map, 4).expect("ups");
    let _obs = lake.reset();

    loop {
        clear_background(BLUE);

        draw_grid(4, 3, vec![false; 12]);
        /*
        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);
        */

        next_frame().await
    }
}
