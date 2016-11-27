use std::mem::size_of;

use lfsr::*;

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct GaloisLFSR {
    pub width: usize,
    pub taps: Vec<usize>,
    tapmasks: Vec<usize>,
    pub state: Vec<usize>,
}

impl GaloisLFSR {
    // Warning: compared to the Fibonacci-style LFSRs this takes bit numbers the other way around.
    // For compatibility with such, use Self::fibonacci().
    #[allow(dead_code)]
    pub fn new(width: usize, taps: Vec<usize>, seed: Vec<usize>) -> Result<GaloisLFSR, String> {
        if width > size_of::<usize>() * 8 {
            return Err(format!("Unsupported: Width {} exceeded that of a single usize.",
                               width));
        }
        let tapmasks = calculate_tapmasks(width, taps.clone())?;
        Ok(GaloisLFSR {
            width: width,
            taps: taps,
            tapmasks: tapmasks,
            state: seed,
        })
    }

    // Instantiate a GaloisLFSR as if it were a Fibonacci-style LFSR. Essentially this method
    // will give a GaloisLFSR whose output will be identical to that from the Fibonacci-style
    // LFSR implementations.
    #[allow(dead_code)]
    pub fn fibonacci(width: usize,
                     taps: Vec<usize>,
                     seed: Vec<usize>)
                     -> Result<GaloisLFSR, String> {
        let mirrored_taps = Self::mirror_taps(width, taps)?;
        Self::new(width, mirrored_taps, seed)
    }

    fn mirror_taps(width: usize, taps: Vec<usize>) -> Result<Vec<usize>, String> {
        if taps.iter().max().unwrap_or(&1) > &width {
            return Err("Tap was outside of register width.".to_string());
        }
        Ok(taps.into_iter().map(|tap| width - tap + 1).collect())
    }

    fn clock_word(mut word: usize, tapmask: usize) -> (usize, usize) {
        let output_bit = word & 1;
        word >>= 1;
        // https://en.wikipedia.org/wiki/Linear-feedback_shift_register#Galois_LFSRs suggested this
        // routine, which I tweaked to suit Rust typing.
        word ^= (-(output_bit as isize) & tapmask as isize) as usize;
        (word, output_bit)
    }
}

impl LFSRTrait for GaloisLFSR {
    fn clock(&mut self) -> usize {
        let (word, output_bit) = Self::clock_word(self.state[0], self.tapmasks[0]);
        self.state[0] = word;
        output_bit
    }

    fn multiclock(&mut self, clocks: usize) -> Vec<usize> {
        let mut outputs = Vec::with_capacity(clocks);
        for _ in 0..clocks {
            outputs.push(self.clock());
        }
        return outputs;
    }

    fn get(&self) -> Vec<usize> {
        self.state.clone()
    }

    fn set(&mut self, value: Vec<usize>) {
        self.state = value;
    }

    fn taps(&self) -> Vec<usize> {
        self.taps.clone()
    }

    fn width(&self) -> usize {
        self.width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ticks_as_expected() {
        let mut naive_lfsr = NaiveLFSR::new(7, vec![1, 2], vec![44]).unwrap();
        let mut galois_lfsr = GaloisLFSR::fibonacci(7, vec![1, 2], vec![44]).unwrap();

        for i in 0..32768 {
            println!("{}", i);
            assert!(naive_lfsr.clock() == galois_lfsr.clock());
        }
    }

    #[test]
    fn rejects_width_larger_than_usize() {
        let usize_bytes = size_of::<usize>();
        let usize_bits = usize_bytes * 8;

        for width in (usize_bits + 1)..(usize_bits * 2 + 1) {
            assert!(GaloisLFSR::fibonacci(width, vec![1], vec![0]).is_err())
        }
    }

    #[test]
    fn rejects_tap_larger_than_register() {
        for width in 0..64 {
            for tap in (width + 1)..64 {
                assert!(GaloisLFSR::fibonacci(width, vec![tap], vec![0]).is_err())
            }
        }
    }
}
