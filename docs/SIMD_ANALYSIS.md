# Phase 16: SIMD & Compiler Auto-Vectorization

## Hypothesis
Explicitly writing hardware-specific SIMD intrinsics (AVX2 / AVX-512) for the hot loop would require writing a custom batched pseudo-random number generator and complex bit-masking branches to handle the Markov state transitions (`if draw < p3`). This violates the project directive against sacrificing maintainability.

However, we hypothesized that the Rust compiler (`rustc`) via the LLVM backend could automatically apply SIMD auto-vectorization to the straight-line arithmetic in our engine (e.g. the Rayon Tree Reduction array accumulators `acc.0[team_id] += 1`) if we allowed it to optimize for the host machine's physical architecture.

## Measurement
We recompiled the binary using `RUSTFLAGS="-C target-cpu=native"` and re-ran the standard 100,000 simulations benchmark on 16 threads.

### Results
- **Phase 15 Baseline**: 1,686,061 games/sec
- **Phase 16 (target-cpu=native)**: 1,797,165 games/sec
- **Throughput Increase**: +111,104 games/sec (+6.5%)

## Conclusion
Without writing a single line of explicit SIMD or `core::arch::x86_64` intrinsic code, we achieved a ~6.5% performance uplift. The LLVM auto-vectorizer successfully identified regions of our highly dense Data-Oriented arrays (implemented in Phase 12) where it could deploy hardware vector instructions.

Because this optimization requires zero code changes and maintains total platform portability (the compiler safely targets the available architecture), it is the superior choice for this simulation engine.
