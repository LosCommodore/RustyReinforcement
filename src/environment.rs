use num_traits::Num;
use std::hash::Hash;

pub struct StepResult<E: Environment + ?Sized> {
    pub observation: E::O,
    pub reward: E::R,
    pub terminated: bool,
    pub truncated: bool,
}

pub trait Environment {
    type A: Copy + Hash + Eq; // Action
    type O: Num + Copy + Hash + Eq; // Observation
    type R: Num + Copy + Hash; // Reward

    fn reset(&mut self) -> Self::O;
    fn step(&mut self, action: Self::A) -> StepResult<Self>;
    fn sample(&self) -> Self::A;
}
