use std::mem::size_of;

use lfsr::*;

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct NaiveLFSR {
    pub width: usize,
    pub taps: Vec<usize>,
    pub state: usize,
}

impl NaiveLFSR {
    #[allow(dead_code)]
    pub fn new(width: usize, taps: Vec<usize>, seed: Vec<usize>) -> Result<NaiveLFSR, String> {
        if width > size_of::<usize>() * 8 {
            return Err(format!("Unsupported: Width {} exceeded that of a single usize.",
                               width));
        }
        if taps.iter().max().unwrap_or(&1) > &width {
            return Err("Tap was outside of register width.".to_string());
        }
        Ok(NaiveLFSR {
            width: width,
            taps: taps,
            state: seed[0],
        })
    }
}

impl LFSRTrait for NaiveLFSR {
    fn clock(&mut self) -> usize {
        let output_bit = self.state & 1;
        let mut feedback_bit = 0;
        for tap in self.taps.iter_mut() {
            // @TODO: Understand exactly why I need to clone tap.
            // Without clone: the trait `std::ops::Shr<&mut usize>` is not implemented for `usize`
            feedback_bit ^= (self.state >> (tap.clone() - 1)) & 1;
        }
        self.state = (self.state >> 1) | (feedback_bit << (self.width - 1));
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
        let l1_exps = vec![1, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 0, 1, 0, 0,
                           1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0];
        ticks_as_expected_prop(7, vec![1, 7], vec![0b100111], l1_exps);

        let l2_exps = vec![1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0,
                           1, 1, 1, 1, 0, 1, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1,
                           1, 1, 1, 0, 1, 0, 1];
        ticks_as_expected_prop(11, vec![1, 10], vec![0b101101101], l2_exps);

        let l3_exps = vec![1, 0, 1, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 0, 0, 1, 0,
                           1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 0, 1,
                           1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0];
        ticks_as_expected_prop(13, vec![1, 10, 11, 13], vec![7413], l3_exps);
    }

    fn ticks_as_expected_prop(width: usize,
                              taps: Vec<usize>,
                              seed: Vec<usize>,
                              expectations: Vec<usize>) {
        let mut lfsr = NaiveLFSR::new(width, taps, seed).unwrap();
        for expectation in expectations {
            assert!(lfsr.clock() == expectation);
        }
    }

    #[test]
    fn rejects_width_larger_than_usize() {
        let usize_bytes = size_of::<usize>();
        let usize_bits = usize_bytes * 8;

        for width in (usize_bits + 1)..(usize_bits * 2 + 1) {
            assert!(NaiveLFSR::new(width, vec![1], vec![0]).is_err())
        }
    }

    #[test]
    fn rejects_tap_larger_than_register() {
        for width in 0..64 {
            for tap in (width + 1)..64 {
                assert!(NaiveLFSR::new(width, vec![tap], vec![0]).is_err())
            }
        }
    }
}
