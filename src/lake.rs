#![allow(unused)]
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
    pos: (usize, usize),
    width: usize,
    height: usize,
    map: Vec<bool>,
}

impl Lake {
    pub fn new(map: &[bool], width: usize) -> Result<Self> {
        if width < 2 {
            bail!("Minimal with is 2")
        }

        if map.len() % width != 0 {
            bail!("Invalid input array, wrong width")
        }

        Ok(Lake {
            width,
            height: map.len() / width,
            map: map.to_vec(),
            pos: (0, 0),
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
        let y2 = (self.pos.0 as isize + y).clamp(0, self.height as isize) as usize;
        let x2 = (self.pos.1 as isize + x).clamp(0, self.width as isize) as usize;
        self.pos = (y2, x2);
    }

    fn is_pit(&self, y: usize, x: usize) -> bool {
        self.map[y * self.width + x]
    }
}

impl Environment for Lake {
    fn reset(&mut self) -> Observation {
        self.pos = (0, 0);
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
