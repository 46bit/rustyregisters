use lfsr::*;

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct NaiveLFSR {
    pub width: usize,
    pub taps: Vec<usize>,
    pub state: usize,
}

impl NaiveLFSR {
    #[allow(dead_code)]
    pub fn new(width: usize, taps: Vec<usize>, seed: Vec<usize>) -> NaiveLFSR {
        NaiveLFSR {
            width: width,
            taps: taps,
            state: seed[0],
        }
    }
}

impl LFSRTrait for NaiveLFSR {
    fn clock(&mut self) -> usize {
        let output_bit = self.state & 1;
        let mut feedback_bit = 0;
        for tap in self.taps.iter_mut() {
            // @TODO: Understand exactly why I need to clone tap.
            // Without clone: the trait `std::ops::Shr<&mut usize>` is not implemented for `usize`
            feedback_bit ^= (self.state >> tap.clone()) & 1;
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
    use quickcheck::{Gen, Arbitrary, quickcheck};
    use super::*;
    pub use lsfr::*;

    impl Arbitrary for NaiveLFSR {
        fn arbitrary<G: Gen>(g: &mut G) -> NaiveLFSR {
            let (width, y) = Arbitrary::arbitrary(g);
            return NaiveLFSR::new(width, taps, seed);
        }
    }

    fn clocks_correctly_prop(l: NaiveLFSR) -> bool {
        unimplemented!("UNsure how to check will tick correctly given mutates NaiveLFSR.")
    }

    #[test]
    fn clocks_correctly() {
        quickcheck(clocks_correctly_prop as fn(NaiveLFSR) -> bool);
    }
}
