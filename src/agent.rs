#![allow(unused)]
use super::lake::Lake;
use crate::environment::{Environment, StepResult};
use crate::lake::Direction;
use std::collections::HashMap;
use std::hash::Hash;
use std::path::Iter;

struct Agent<E: Environment> {
    random_executor: RandomActionExecutor<E>,
    rewards: HashMap<Transition<E::O, E::A>, E::R>,
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Transition<O: Eq + Hash, A: Eq + Hash> {
    source: O,
    dest: O,
    action: A,
}

struct RandomActionExecutor<E: Environment> {
    env: E,
    state: E::O,
}

/// Executes a random action on its environment infinity and resets the
/// environment always when it is finished.
impl<E: Environment> RandomActionExecutor<E> {
    fn new(mut env: E) -> Self {
        let observation = env.reset();
        Self {
            env,
            state: observation,
        }
    }

    pub fn random_step(&mut self) -> (Transition<E::O, E::A>, E::R) {
        let action = self.env.sample();

        let StepResult {
            observation,
            reward,
            terminated,
            truncated,
        } = self.env.step(action);

        let transition = Transition {
            source: self.state,
            dest: observation,
            action,
        };

        self.state = if terminated || truncated {
            self.env.reset()
        } else {
            observation
        };

        (transition, reward)
    }
}

impl<E: Environment> Iterator for RandomActionExecutor<E> {
    type Item = (Transition<E::O, E::A>, E::R);

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
