use crate::environment::{Environment, StepResult};
use std::collections::HashMap;
use std::hash::Hash;

type Counter<T> = HashMap<T, usize>;

#[allow(unused)]
pub struct QIterAgent<E: Environment> {
    random_executor: RandomActionExecutor<E>,
    transition_counters: HashMap<TransitionOf<E>, Counter<E::O>>,
    rewards: HashMap<Route<E::O, E::A>, E::R>,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Route<O: Eq + Hash, A: Eq + Hash> {
    pub source: O,
    pub dest: O,
    pub action: A,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Transition<O: Eq + Hash, A: Eq + Hash> {
    pub source: O,
    pub action: A,
}

type TransitionOf<E> = Transition<<E as Environment>::O, <E as Environment>::A>;

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

    pub fn random_step(&mut self) -> (Route<E::O, E::A>, E::R) {
        let action = self.env.sample();

        let StepResult {
            observation,
            reward,
            terminated,
            truncated,
        } = self.env.step(action);

        let transition = Route {
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
    type Item = (Route<E::O, E::A>, E::R);

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.random_step())
    }
}

impl<E: Environment> QIterAgent<E> {
    pub fn new(env: &E) -> Self {
        Self {
            random_executor: RandomActionExecutor::new(env.clone()),
            transition_counters: HashMap::new(),
            rewards: HashMap::new(),
        }
    }

    pub(crate) fn transition_counters(&self) -> &HashMap<TransitionOf<E>, Counter<E::O>> {
        &self.transition_counters
    }

    pub(crate) fn rewards(&self) -> &HashMap<Route<E::O, E::A>, E::R> {
        &self.rewards
    }

    pub fn play_random(&mut self, nr_steps: usize) {
        for _ in 0..nr_steps {
            let (route, reward) = self.random_executor.random_step();
            let transition = Transition {
                source: route.source,
                action: route.action,
            };

            self.rewards.insert(route.clone(), reward);
            *self
                .transition_counters
                .entry(transition)
                .or_default()
                .entry(route.dest)
                .or_default() += 1;
        }
    }
}
