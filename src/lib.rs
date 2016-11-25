#![feature(asm, proc_macro)]

extern crate ansi_term;
#[macro_use(chan_select)]
extern crate chan;
extern crate rand;
extern crate rayon;
#[cfg(test)]
extern crate quickcheck;

mod lfsr;

pub use lfsr::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
