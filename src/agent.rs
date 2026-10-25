#![allow(unused)]
use super::lake::Lake;
use crate::environment::{Environment, StepResult};
use crate::lake::Direction;
use std::collections::HashMap;
use std::hash::Hash;
use std::path::Iter;

struct Agent<E: Environment> {
    random_executor: RandomActionExecutor,
    rewards: HashMap<Transition<E::O, E::A>, E::R>,
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Transition<O: Eq + Hash, A: Eq + Hash> {
    source: O,
    dest: O,
    action: A,
}

struct RandomActionExecutor {
    lake: Lake,
    state: usize,
}

/// Executes a random action on its environment infinity and resets the
/// environment always when it is finished.
impl RandomActionExecutor {
    fn new(mut lake: Lake) -> Self {
        let observation = lake.reset();
        Self {
            lake,
            state: observation,
        }
    }

    pub fn random_step(&mut self) -> (Transition<usize, Direction>, usize) {
        let action = self.lake.sample();

        let StepResult {
            observation,
            reward,
            terminated,
            truncated,
        } = self.lake.step(action);

        let transition = Transition {
            source: self.state,
            dest: observation,
            action,
        };

        self.state = if terminated || truncated {
            self.lake.reset()
        } else {
            observation
        };

        (transition, reward)
    }
}

impl Iterator for RandomActionExecutor {
    type Item = (Transition<usize, Direction>, usize);

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.random_step())
    }
}

impl<E: Environment> Agent<E> {
    fn play_n_random_steps(&mut self, env: &mut E, nr_steps: usize) {
        for s in 0..nr_steps {
            let (transition, reward) = self.random_executor.random_step();
            self.rewards.insert(transition, reward);
        }
    }
}
