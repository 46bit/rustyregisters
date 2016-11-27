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
    let taps = vec![64, 63, 61, 60];

    let seeds = two.pow(16);
    let clocks = two.pow(16);

    let galois = GaloisLFSR::fibonacci(width, taps.clone(), vec![0]).unwrap();
    let galois_duration = time(galois, seeds, clocks);
    println!("{} took {:?}ms to perform 2^32 clockings.",
             Purple.bold().paint("GaloisLFSR"),
             galois_duration.num_milliseconds());

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    let pop_cnt = PopCntLFSR::new(width, taps.clone(), vec![0]).unwrap();
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    let pop_cnt_duration = time(pop_cnt, seeds, clocks);
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    println!("{} took {:?}ms to perform 2^32 clockings.",
             Purple.bold().paint("PopCntLFSR"),
             pop_cnt_duration.num_milliseconds());

    let count_ones = CountOnesLFSR::new(width, taps.clone(), vec![0]).unwrap();
    let count_ones_duration = time(count_ones, seeds, clocks);
    println!("{} took {:?}ms to perform 2^32 clockings.",
             Purple.bold().paint("CountOnesLFSR"),
             count_ones_duration.num_milliseconds());

    let naive = NaiveLFSR::new(width, taps.clone(), vec![0]).unwrap();
    let naive_duration = time(naive, seeds, clocks);
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
