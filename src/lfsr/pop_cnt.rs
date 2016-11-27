use std::mem::size_of;

use lfsr::*;

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct PopCntLFSR {
    pub width: usize,
    pub taps: Vec<usize>,
    tapmasks: Vec<usize>,
    pub state: usize,
}

impl PopCntLFSR {
    #[allow(dead_code)]
    pub fn new(width: usize, taps: Vec<usize>, seed: Vec<usize>) -> Result<PopCntLFSR, String> {
        if width > size_of::<usize>() * 8 {
            return Err(format!("Unsupported: Width {} exceeded that of a single usize.",
                               width));
        }
        let tapmasks = calculate_tapmasks(width, taps.clone())?;
        Ok(PopCntLFSR {
            width: width,
            taps: taps,
            tapmasks: tapmasks,
            state: seed[0],
        })
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
        let tapped = self.state & self.tapmasks[0];
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
    use super::*;

    #[test]
    fn ticks_as_expected() {
        let mut naive_lfsr = NaiveLFSR::new(7, vec![1, 2], vec![44]).unwrap();
        let mut pop_cnt_lfsr = PopCntLFSR::new(7, vec![1, 2], vec![44]).unwrap();

        for _ in 0..32768 {
            assert!(naive_lfsr.clock() == pop_cnt_lfsr.clock());
        }
    }

    #[test]
    fn rejects_width_larger_than_usize() {
        let usize_bytes = size_of::<usize>();
        let usize_bits = usize_bytes * 8;

        for width in (usize_bits + 1)..(usize_bits * 2 + 1) {
            assert!(PopCntLFSR::new(width, vec![1], vec![0]).is_err())
        }
    }

    #[test]
    fn rejects_tap_larger_than_register() {
        for width in 0..64 {
            for tap in (width + 1)..64 {
                assert!(PopCntLFSR::new(width, vec![tap], vec![0]).is_err())
            }
        }
    }
}
