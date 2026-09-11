# RNG & Deterministic Parallelism Architecture

## 1. The Core Problem
Monte Carlo simulations typically require millions of parallel paths to converge on a valid probability distribution. However, typical approaches to Random Number Generators (RNGs) in parallel execution introduce severe bugs:

- **Shared State PRNGs** (e.g. `Arc<Mutex<Rng>>`): Creating a single global RNG and placing it behind a lock destroys multi-threaded performance through extreme lock contention.
- **Thread-Local PRNGs** (e.g. `ThreadRng`): Using thread-local generators means that the stream of randomness a simulation path receives depends *entirely* on the OS thread scheduler. Running `./nba-sim --seed 42 --threads 1` and `./nba-sim --seed 42 --threads 8` will yield entirely different statistical results, making debugging, unit testing, and benchmarking completely irreproducible.

## 2. Our Solution: Hierarchical Splittable Streams
To achieve strictly deterministic execution, the random stream must be an pure function of the simulation's topological position, completely independent of *when* or *where* it is executed.

We use **PCG64** (`rand_pcg::Pcg64Mcg`), an ultra-fast, statistically robust Linear Congruential Generator (LCG) combined with a Permuted Congruential Generator output function.

### A. The Avalanche Seeder
To prevent stream overlaps (which happens when two seeds are extremely similar), we use a high-entropy `SplitMix64` avalanche function to strictly uncorrelate the identifiers before passing them to the PCG seeder:

```rust
let mut combined = Self::splitmix64(global_seed);
combined ^= Self::splitmix64(sim_id);
combined ^= Self::splitmix64(game_id);
let final_seed = Self::splitmix64(combined);
```

### B. The Property of Determinism
With this structure, the exact same possession sequence for Game $X$ in Series $Y$ of Simulation $Z$ will be generated regardless of whether it is computed first on Core 0 or last on Core 64. 

The `verify-determinism` suite empirically proves this by executing 50,000 simulations under Rayon work-stealing pools bounded to `1, 2, 4, 8, 16` threads and validating that the output accumulator arrays are 100% bit-for-bit identical.
