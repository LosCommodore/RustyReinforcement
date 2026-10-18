#![feature(gen_blocks)]

mod lake;
mod matrix;
use anyhow::Result;
use lake::Environment;
use lake::Lake;

const X: bool = true;
const O: bool = false;

use macroquad::prelude::*;

use crate::matrix::Matrix;

/// Iterator returning grid positions ((ix, iy), (x, y))
fn iter_grid(
    dim_x: usize,
    dim_y: usize,
    w_cell: f32,
    h_cell: f32,
) -> impl Iterator<Item = ((usize, usize), (f32, f32))> {
    gen move {
        for iy in 0..dim_y {
            for ix in 0..dim_x {
                let x = (ix as f32 * w_cell) as f32;
                let y = (iy as f32 * h_cell) as f32;
                yield ((ix, iy), (x, y))
            }
        }
    }
}

fn draw_grid(map: &Matrix<bool>) {
    let w_cell = screen_width() / map.width() as f32;
    let h_cell = screen_height() / map.height() as f32;

    let it = iter_grid(map.width(), map.height(), w_cell, h_cell);
    for ((ix, iy), (x, y)) in it {
        let color = if map[(iy,ix)] == true {
            GREEN
        } else {
            RED
        };

        draw_rectangle(x, y, w_cell, h_cell, color);
        draw_rectangle_lines(x, y, w_cell, h_cell, 2., BLACK);
    }
}

async fn main_loop(lake: &Lake) {
    loop {
        clear_background(BLUE);
        draw_grid(lake.map());
        next_frame().await
    }
}

#[macroquad::main("MyGame")]
async fn main() -> Result<()> {
    #[rustfmt::skip]
    let map: Vec<bool> = vec![
        O, O, O, O,
        O, X, O, X, 
        O, O, O, X, 
        X, O, O, O
        ];

    let map: Matrix<bool> = Matrix::from_vec(map, 4, 4)?;

    let mut lake: Lake = Lake::new(map)?;
    let _obs = lake.reset();
    main_loop(&lake).await;
    Ok(())
}
