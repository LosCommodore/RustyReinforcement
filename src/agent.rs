#![allow(unused)]

use crate::Environment;

use super::lake::Lake;

struct Agent<E: Environment> {
    current_state: E::O,
}

impl<E: Environment> Agent<E> {
    fn play_n_random_steps(&self, env: &mut impl Environment) {
        let action = env.sample();
        let step = env.step(action);
        let transition = (self.current_state, step.observation, action);
        todo!()
    }
}
