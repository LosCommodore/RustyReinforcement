#![allow(unused)]

type Action = Direction;
type Observation = usize;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn orthogonal(&self) -> [Direction; 2] {
        todo!()
    }

    fn vector() -> (usize, usize) {
        todo!()
    }
}

struct StepResult {
    observation: Observation,
    reward: usize,
    terminated: bool,
    truncated: bool,
}

trait Environment {
    fn reset() -> usize;
    fn step(action: Action) -> StepResult;
}

struct Lake {
    pos: (usize, usize),
    width: usize,
    height: usize,
    map: Vec<bool>,
}

impl Lake {
    fn new() -> Self {
        let width = 4;
        let height = 4;
        let map = vec![false; width * height];
        let pos = 0;

        // set some random pits

        Lake {
            width,
            height,
            map,
            pos,
        }
    }

    fn get_walk_direction(&self, direction: Direction) -> Direction {
        let p: usize = 1; // random integer from 0,1,2

        if p == 2 {
            return direction;
        }

        direction.orthogonal()[p]
    }

    fn walk(&mut self, direction: Direction) {
        let d = self.get_walk_direction(direction);

        // bound ? -> do nothing
    }
}

impl Environment for Lake {
    fn reset() -> usize {
        0
    }

    fn step(action: Action) -> StepResult {
        StepResult {
            observation: 0,
            reward: 0,
            terminated: false,
            truncated: false,
        }
    }
}
