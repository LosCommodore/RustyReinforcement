#![allow(unused)]
use super::lake::Lake;
use crate::environment::Environment;
use std::collections::HashMap;
use std::hash::Hash;

struct Agent<E: Environment> {
    current_state: E::O,
    rewards: HashMap<Transition<E::O, E::A>, E::R>,
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Transition<O: Eq + Hash, A: Eq + Hash> {
    source: O,
    dest: O,
    action: A,
}

impl<E: Environment> Agent<E> {
    fn play_n_random_steps(&mut self, env: &mut E) {
        let action = env.sample();
        let step = env.step(action);
        let transition = Transition {
            source: self.current_state,
            dest: step.observation,
            action,
        };
        self.rewards.insert(transition, step.reward);
    }
}
