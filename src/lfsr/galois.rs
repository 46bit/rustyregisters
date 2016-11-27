use lfsr::*;

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct GaloisLFSR {
    pub width: usize,
    pub taps: Vec<usize>,
    tapmask: usize,
    pub state: usize,
}

impl GaloisLFSR {
    // Warning: compared to the Fibonacci-style LFSRs this takes bit numbers the other way around.
    // For compatibility with such, use Self::fibonacci().
    #[allow(dead_code)]
    pub fn new(width: usize, taps: Vec<usize>, seed: Vec<usize>) -> GaloisLFSR {
        GaloisLFSR {
            width: width,
            taps: taps.clone(),
            tapmask: Self::calculate_tapmask(taps),
            state: seed[0],
        }
    }

    // Instantiate a GaloisLFSR as if it were a Fibonacci-style LFSR. Essentially this method
    // will give a GaloisLFSR whose output will be identical to that from the Fibonacci-style
    // LFSR implementations.
    #[allow(dead_code)]
    pub fn fibonacci(width: usize, taps: Vec<usize>, seed: Vec<usize>) -> GaloisLFSR {
        let mirrored_taps = Self::mirror_taps(width, taps);
        Self::new(width, mirrored_taps, seed)
    }

    fn calculate_tapmask(taps: Vec<usize>) -> usize {
        let mut tapmask = 0;
        for tap in taps {
            tapmask |= (1 as usize) << (tap - 1);
        }
        return tapmask;
    }

    fn mirror_taps(width: usize, taps: Vec<usize>) -> Vec<usize> {
        taps.into_iter().map(|tap| width - tap + 1).collect()
    }
}

impl LFSRTrait for GaloisLFSR {
    fn clock(&mut self) -> usize {
        let output_bit = self.state & 1;
        self.state >>= 1;
        // https://en.wikipedia.org/wiki/Linear-feedback_shift_register#Galois_LFSRs suggested this
        // routine, which I tweaked to suit Rust typing.
        self.state ^= (-(output_bit as isize) & self.tapmask as isize) as usize;
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
        vec![self.state]
    }

    fn set(&mut self, value: Vec<usize>) {
        self.state = value[0];
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
        let mut naive_lfsr = NaiveLFSR::new(7, vec![1, 2], vec![44]);
        let mut galois_lfsr = GaloisLFSR::fibonacci(7, vec![1, 2], vec![44]);

        for i in 0..32768 {
            println!("{}", i);
            assert!(naive_lfsr.clock() == galois_lfsr.clock());
        }
    }
}
