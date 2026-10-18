#![allow(unused)]
use super::matrix::Matrix;
use anyhow::{Result, bail};
type Action = Direction;
type Observation = usize;

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn orthogonal(&self) -> [Direction; 2] {
        use Direction::*;
        match self {
            Up | Down => [Left, Right],
            Left | Right => [Up, Down],
        }
    }

    fn vector(&self) -> (isize, isize) {
        use Direction::*;
        match self {
            Up => (-1, 0),
            Down => (1, 0),
            Right => (0, 1),
            Left => (0, -1),
        }
    }
}

pub struct StepResult {
    observation: Observation,
    reward: usize,
    terminated: bool,
    truncated: bool,
}

pub trait Environment {
    fn reset(&mut self) -> Observation;
    fn step(&mut self, action: Action) -> StepResult;
}

pub struct Lake {
    player_pos: (usize, usize),
    map: Matrix<bool>,
}

impl Lake {
    pub fn new(map: Matrix<bool>) -> Result<Self> {
        if map.width() < 2 {
            bail!("Minimal with is 2")
        }

        Ok(Lake {
            map: map,
            player_pos: (0, 0),
        })
    }

    fn get_walk_direction(&self, direction: Direction) -> Direction {
        let p: usize = rand::random_range(0..=2);

        if p == 2 {
            return direction;
        }

        direction.orthogonal()[p]
    }

    fn walk(&mut self, direction: Direction) {
        let d = self.get_walk_direction(direction);
        let (y, x) = d.vector();
        let y2 = (self.player_pos.0 as isize + y).clamp(0, self.map.height() as isize) as usize;
        let x2 = (self.player_pos.1 as isize + x).clamp(0, self.map.width() as isize) as usize;
        self.player_pos = (y2, x2);
    }

    fn is_pit(&self, y: usize, x: usize) -> bool {
        self.map[(y * self.map.width(), x)]
    }
}

impl Environment for Lake {
    fn reset(&mut self) -> Observation {
        self.player_pos = (0, 0);
        0
    }

    fn step(&mut self, action: Action) -> StepResult {
        StepResult {
            observation: 0,
            reward: 0,
            terminated: false,
            truncated: false,
        }
    }
}
