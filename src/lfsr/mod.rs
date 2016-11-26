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
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod pop_cnt;
pub mod galois;

pub use naive::*;
pub use count_ones::*;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub use pop_cnt::*;
pub use galois::*;
