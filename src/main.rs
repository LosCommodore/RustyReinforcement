#![feature(gen_blocks)]
mod agent;
mod dot;
mod draw;
mod environment;
mod lake;
mod matrix;
use crate::agent::QIterAgent;
use crate::dot::DotRenderer;
use crate::draw::draw_figure;
use crate::environment::StepResult;
use crate::matrix::Matrix;
use environment::Environment;
use macroquad::prelude::*;

use anyhow::Result;
use draw::draw_grid;
use lake::Lake;

const X: bool = true;
const O: bool = false;

async fn display_loop(lake: &mut Lake) -> bool {
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

    let mut lake: Lake = Lake::new(map, 200)?;
    let mut agent = QIterAgent::new(&lake);
    agent.play_random(100);

    let dr = DotRenderer::new()?;
    let render_txt = dr.render_agent(agent);
    std::fs::write("agent.dot", render_txt)?;
    std::process::Command::new("dot")
        .args(["-Tpng", "agent.dot", "-o", "agent.png"])
        .status()?;

    display_loop(&mut lake).await;
    Ok(())
}
