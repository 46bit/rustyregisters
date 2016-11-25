use lfsr::*;

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct PopCntLFSR {
    pub width: usize,
    pub taps: Vec<usize>,
    pub tapmask: usize,
    pub state: usize,
}

impl PopCntLFSR {
    #[allow(dead_code)]
    pub fn new(width: usize, taps: Vec<usize>, seed: Vec<usize>) -> PopCntLFSR {
        PopCntLFSR {
            width: width,
            taps: taps.clone(),
            tapmask: PopCntLFSR::calculate_tapmask(taps),
            state: seed[0],
        }
    }

    fn calculate_tapmask(taps: Vec<usize>) -> usize {
        let mut tapmask = 0;
        for tap in taps {
            tapmask |= (1 as usize) << tap;
        }
        return tapmask;
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
// Note you can pipeline 4 POPCNT totalling 256 bits:
//   http://danluu.com/assembly-intrinsics/
fn u64_popcnt_instruction(value: u64) -> u32 {
    let result: u32;
    unsafe {
        asm!("popcnt $0, $1"
            : "=r"(result)
            : "0"(value));
    }
    result
}

impl LFSRTrait for PopCntLFSR {
    fn clock(&mut self) -> usize {
        let output_bit = self.state & 1;
        let tapped = self.state & self.tapmask;
        let feedback_bit = (u64_popcnt_instruction(tapped as u64) & 1) as usize;
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

    impl Arbitrary for PopCntLFSR {
        fn arbitrary<G: Gen>(g: &mut G) -> PopCntLFSR {
            let (width, y) = Arbitrary::arbitrary(g);
            return PopCntLFSR::new(width, taps, seed);
        }
    }

    fn clocks_correctly_prop(l: PopCntLFSR) -> bool {
        unimplemented!("UNsure how to check will tick correctly given mutates PopCntLFSR.")
    }

    #[test]
    fn clocks_correctly() {
        quickcheck(clocks_correctly_prop as fn(PopCntLFSR) -> bool);
    }
}
