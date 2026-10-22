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

use crate::draw::draw_figure;
use crate::lake::StepResult;
use crate::matrix::Matrix;
use macroquad::prelude::*;

async fn main_loop(lake: &mut Lake) -> bool {
    lake.reset();
    loop {
        clear_background(BLUE);
        draw_grid(lake.map());
        draw_figure(lake.map(), lake.player_pos());

        let StepResult {
            reward,
            terminated,
            truncated,
            ..
        } = lake.step(lake.sample());

        if truncated {
            return false;
        }

        if terminated {
            lake.reset();
        }

        if reward == 1 {
            return true;
        }

        std::thread::sleep(std::time::Duration::from_millis(500));

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

    let mut lake: Lake = Lake::new(map, 100)?;
    main_loop(&mut lake).await;
    Ok(())
}
