use std::fmt::Debug;
use std::hash::Hash;
use std::mem::size_of;

pub trait LFSRTrait: PartialEq + Eq + Clone + Hash + Debug {
    fn clock(&mut self) -> usize;
    fn multiclock(&mut self, clocks: usize) -> Vec<usize>;
    fn get(&self) -> Vec<usize>;
    fn set(&mut self, value: Vec<usize>);
    fn taps(&self) -> Vec<usize>;
    fn width(&self) -> usize;
}

fn width_in_usize_words(width: usize) -> usize {
    let usize_bytes = size_of::<usize>();
    let usize_bits = usize_bytes * 8;
    (width + usize_bits - 1) / usize_bits
}

fn calculate_tapmasks(width: usize, taps: Vec<usize>) -> Result<Vec<usize>, String> {
    let usize_bytes = size_of::<usize>();
    let usize_bits = usize_bytes * 8;
    let words = width_in_usize_words(width);

    let mut tapmasks = Vec::with_capacity(words);
    for _ in 0..words {
        tapmasks.push(0);
    }

    for tap in taps {
        if tap > width {
            return Err(format!("Tap {} was outside of register width {}.", tap, width));
        }
        tapmasks[(tap - 1) / usize_bits] |= (1 as usize) << (tap - 1);
    }
    Ok(tapmasks)
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
