extern crate ansi_term;
#[macro_use(chan_select)]
extern crate chan;
extern crate rand;
extern crate rayon;
extern crate rustyregisters;

use ansi_term::Colour::*;
use std::io::{self, Write};

use rustyregisters::*;

fn main() {
    println!("{} {}",
             Yellow.bold().paint("RustyRegisters"),
             Red.bold().paint("LFSR Benchmark"));

    let two = 2 as usize;
    let width = 13;
    let taps = vec![0, 3, 7, 12];

    let width = 64;
    let taps = vec![64 - 1, 63 - 1, 61 - 1, 60 - 1];

    // let mut naive = NaiveLFSR::new(width, taps.clone(), vec![0]);
    // for seed in 0..two.pow(16) {
    //     naive.set(vec![seed]);
    //     naive.multiclock(two.pow(14));
    // }

    let mut count_ones = CountOnesLFSR::new(width, taps.clone(), vec![0]);
    for seed in 0..two.pow(16) {
        count_ones.set(vec![seed]);
        count_ones.multiclock(two.pow(14));
    }
}
