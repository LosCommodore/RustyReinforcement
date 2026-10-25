#![allow(unused)]
use crate::environment::{Environment, StepResult};

use super::matrix::Matrix;
use anyhow::{Result, bail};
use core::num;
use rand::seq::IndexedRandom;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Action = Direction;
type Observation = usize;

impl Direction {
    const ALL: [Action; 4] = [Action::Left, Action::Down, Action::Right, Action::Up];

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

pub struct Lake {
    player_pos: (usize, usize),
    map: Matrix<bool>,
    step_counter: usize,
    max_steps: usize,
}

impl Lake {
    pub fn new(map: Matrix<bool>, max_steps: usize) -> Result<Self> {
        if map.width() < 2 {
            bail!("Minimal with is 2")
        }

        Ok(Lake {
            map,
            player_pos: (0, 0),
            step_counter: 0,
            max_steps,
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

    pub fn map(&self) -> &Matrix<bool> {
        &self.map
    }

    pub fn player_pos(&self) -> (usize, usize) {
        self.player_pos
    }
}

impl Environment for Lake {
    type A = Action;
    type O = Observation;
    type R = usize;

    fn reset(&mut self) -> Observation {
        self.player_pos = (0, 0);
        0
    }

    fn step(&mut self, action: Action) -> StepResult<Self> {
        self.walk(action);

        let observation = self.player_pos.0 * self.map.width() + self.player_pos.1;
        let num_fields = self.map().width() * self.map.height();
        self.step_counter += 1;

        StepResult {
            observation,
            reward: (observation == num_fields - 1) as usize,
            terminated: self.map()[self.player_pos],
            truncated: self.step_counter >= self.max_steps,
        }
    }

    fn sample(&self) -> Action {
        let mut rng = rand::rng();
        *Action::ALL.choose(&mut rng).unwrap()
    }
}
