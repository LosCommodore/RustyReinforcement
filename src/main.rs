#![feature(gen_blocks)]
mod agent;
mod draw;
mod lake;
mod matrix;

use crate::draw::draw_figure;
use crate::matrix::Matrix;
use macroquad::prelude::*;
use num_traits::Num;

use anyhow::Result;
use draw::draw_grid;
use lake::Lake;

const X: bool = true;
const O: bool = false;

pub struct StepResult<T> {
    pub observation: T,
    pub reward: usize,
    pub terminated: bool,
    pub truncated: bool,
}

pub trait Environment {
    type A: Copy; // Action
    type O: Num + Copy; // Observation

    fn reset(&mut self) -> Self::O;
    fn step(&mut self, action: Self::A) -> StepResult<Self::O>;
    fn sample(&self) -> Self::A;
}

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
