extern crate ansi_term;
#[macro_use(chan_select)]
extern crate chan;
extern crate rand;
extern crate rayon;
extern crate time;
extern crate rustyregisters;

use ansi_term::Colour::*;
use time::Duration;

use rustyregisters::*;

fn main() {
    println!("{} {}",
             Yellow.bold().paint("RustyRegisters"),
             Red.bold().paint("LFSR Benchmark"));

    let two = 2 as usize;
    let width = 64;
    let taps = vec![64 - 1, 63 - 1, 61 - 1, 60 - 1];

    let seeds = two.pow(16);
    let clocks = two.pow(16);

    let galois_duration = time(GaloisLFSR::new(width, taps.clone(), vec![0]), seeds, clocks);
    println!("{} took {:?}ms to perform 2^32 clockings.",
             Purple.bold().paint("GaloisLFSR"),
             galois_duration.num_milliseconds());

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    let pop_cnt_duration = time(PopCntLFSR::new(width, taps.clone(), vec![0]), seeds, clocks);
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    println!("{} took {:?}ms to perform 2^32 clockings.",
             Purple.bold().paint("PopCntLFSR"),
             pop_cnt_duration.num_milliseconds());

    let count_ones_duration = time(CountOnesLFSR::new(width, taps.clone(), vec![0]),
                                   seeds,
                                   clocks);
    println!("{} took {:?}ms to perform 2^32 clockings.",
             Purple.bold().paint("CountOnesLFSR"),
             count_ones_duration.num_milliseconds());

    let naive_duration = time(NaiveLFSR::new(width, taps.clone(), vec![0]), seeds, clocks);
    println!("{} took {:?}ms to perform 2^32 clockings.",
             Purple.bold().paint("NaiveLFSR"),
             naive_duration.num_milliseconds());
}

fn time<L: LFSRTrait>(mut lfsr: L, seeds: usize, clocks: usize) -> Duration {
    Duration::span(move || {
        for seed in 0..seeds {
            lfsr.set(vec![seed]);
            lfsr.multiclock(clocks);
        }
    })
}
