#![feature(gen_blocks)]

mod draw;
mod lake;
mod matrix;
use anyhow::Result;
use draw::draw_grid;
use lake::Environment;
use lake::Lake;

const X: bool = true;
const O: bool = false;

use crate::matrix::Matrix;
use macroquad::prelude::*;

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
