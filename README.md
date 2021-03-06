# RustyRegisters

**Linear Feedback Shift Register (LFSR) experiments in Rust.**

## LFSR Benchmark
`src/bin/lfsr_benchmark.rs`: Test different implementations of a Linear Feedback Shift Register.

* **NaiveLFSR**: Naive Fibonacci LFSR implementation.
  Each clock operations takes time `O(t)` in the number of taps.

  ``` rust
  let mut feedback_bit = 0;
  for tap in self.taps.iter_mut() {
      feedback_bit ^= (self.state >> tap.clone()) & 1;
  }
  ```
* **CountOnesLFSR**: LSB-of-Population-Count Fibonacci LFSR implementation.
    With [Rust's default shift-add chain](https://users.rust-lang.org/t/what-is-the-implementation-of-count-ones/4923) this is still `O(t)` in the number of taps. If using a native `POPCNT` instruction this should be `O(1)` in the number of taps.

    ``` rust
    let tapped = self.state & self.tapmask;
    let feedback_bit = (tapped.count_ones() & 1) as usize;
    ```
* **PopCntLFSR**: inline Assembly x86_64 POPCNT for LSB-of-Population-Count Fibonacci LFSR implementation.
    This is firmly `O(1)` in the number of taps.

    ``` rust
    let feedback_bit = (u64_popcnt_instruction(tapped as u64) & 1) as usize;
    ```
* **GaloisLFSR**: alternative form capable of giving the same output as ordinary Fibonacci LFSRs. Very fast in software: Feedback becomes a single XOR operation per word.
    This is firmly `O(1)` in the number of taps.

    ``` rust
    self.state ^= (-(output_bit as isize) & self.tapmask as isize) as usize;
    ```

### Performance

Time taken for 2<sup>32</sup> clockings on a 64-bit register with the polynomial x<sup>63</sup> + x<sup>62</sup> + x<sup>60</sup> + x<sup>59</sup> + 1`. Timings obtained on a 2.2GHz 2015 15" MBP.

| Implementation | Unoptimising compile | Optimising compile | Optimising & `target-feature=+popcnt` |
| :---           |                 ---: |               ---: |                                  ---: |
| NaiveLFSR      |          1,498,100ms |           30,000ms |                              29,000ms |
| CountOnesLFSR  |            456,000ms |           31,000ms |                              17,800ms |
| PopCntLFSR     |            450,000ms |           18,500ms |                              17,900ms |
| GaloisLFSR     |        **410,628ms** |       **13,000ms** |                          **13,000ms** |

Unoptimising compile: `cargo run`
Optimising compile: `cargo run --release`
Optimising compile with `target-feature=+popcnt`: `RUSTFLAGS="-C target-feature=+popcnt" cargo run --release`

The `target-feature=+popcnt` flag enables the `rustc` compiler to use `POPCNT` instructions. As we can see, the resulting optimisation of `CountOnesLFSR` accelerates it slightly past my `POPCNT`-always `PopCntLFSR`.
