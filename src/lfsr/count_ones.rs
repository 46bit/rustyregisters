use lfsr::*;

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct CountOnesLFSR {
    pub width: usize,
    pub taps: Vec<usize>,
    tapmask: usize,
    pub state: usize,
}

impl CountOnesLFSR {
    #[allow(dead_code)]
    pub fn new(width: usize, taps: Vec<usize>, seed: Vec<usize>) -> CountOnesLFSR {
        CountOnesLFSR {
            width: width,
            taps: taps.clone(),
            tapmask: Self::calculate_tapmask(taps),
            state: seed[0],
        }
    }

    fn calculate_tapmask(taps: Vec<usize>) -> usize {
        let mut tapmask = 0;
        for tap in taps {
            tapmask |= (1 as usize) << (tap - 1);
        }
        return tapmask;
    }
}

impl LFSRTrait for CountOnesLFSR {
    fn clock(&mut self) -> usize {
        let output_bit = self.state & 1;
        let tapped = self.state & self.tapmask;
        let feedback_bit = (tapped.count_ones() & 1) as usize;
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
        let mut naive_lfsr = NaiveLFSR::new(7, vec![1, 2], vec![44]);
        let mut count_ones_lfsr = CountOnesLFSR::new(7, vec![1, 2], vec![44]);

        for _ in 0..32768 {
            assert!(naive_lfsr.clock() == count_ones_lfsr.clock());
        }
    }
}
