use std::fmt::Debug;
use std::hash::Hash;

pub trait LFSRTrait: PartialEq + Eq + Clone + Hash + Debug {
    fn clock(&mut self) -> usize;
    fn multiclock(&mut self, clocks: usize) -> Vec<usize>;
    fn get(&self) -> Vec<usize>;
    fn set(&mut self, value: Vec<usize>);
    fn taps(&self) -> Vec<usize>;
    fn width(&self) -> usize;
}

pub mod naive;
pub mod count_ones;

pub use naive::*;
pub use count_ones::*;
