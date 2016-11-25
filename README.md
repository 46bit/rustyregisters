# RustyRegisters

**Linear Feedback Shift Register (LFSR) experiments in Rust.**

## LFSR Benchmark
`src/bin/lfsr_benchmark.rs`: Test different implementations of a Linear Feedback Shift Register.

* **NaiveLFSR**: Naive Fibonacci LFSR implementation.
  If unoptimised each clock is `O(t)` in the number of taps.
  Optimised run time: 7.01s.
  ``` rust
  let mut feedback_bit = 0;
  for tap in self.taps.iter_mut() {
      feedback_bit ^= (self.state >> tap.clone()) & 1;
  }
  ```
* **CountOnesLFSR**: LSB-of-Population-Count Fibonacci LFSR implementation.
  * If using [Rust's default shift-add chain](https://users.rust-lang.org/t/what-is-the-implementation-of-count-ones/4923) this is a cleaner `O(t)` in the number of taps.
    Optimised run time: 8.10s.
    ``` rust
    let tapped = self.state & self.tapmask;
    let feedback_bit = (tapped.count_ones() & 1) as usize;
    ```
  * If using a native `POPCNT` instruction this should be `O(1)` in the number of taps.
    Optimised run time: 4.40s.

### Running benchmarks

* NaiveLFSR: Uncomment relevant loop. Run optimised compile with `cargo build --release` then time with `time cargo run --release`.
* CountOnesLFSR (no POPCNT): Run optimised compile with `cargo build --release` then time with `time cargo run --release`.
* CountOnesLFSR (with POPCNT): Run optimised compile with `RUSTFLAGS="-C target-feature=+popcnt" cargo build --release` then time with `time RUSTFLAGS="-C target-feature=+popcnt" cargo run --release`.
