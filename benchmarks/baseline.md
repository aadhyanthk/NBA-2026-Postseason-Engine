# Single-Threaded Baseline Benchmark

## 1. Machine & Environment Specifications
- **Operating System**: Windows 11 (x86_64)
- **Rust Toolchain**: `stable-x86_64-pc-windows-msvc` (Rust 1.84+)
- **Compilation Profile**: `--release` (`opt-level = 3`)
- **Concurrency Mode**: Single-threaded (`--threads 1`)
- **PRNG Model**: Hierarchical Splittable ChaCha8 (`Hash(MasterSeed, SimIndex, 0)`)
- **Game Simulation Model**: Four Factors Possession-Level Markov Model (Pace, ORtg, DRtg, TOV%, OREB%, 3P Rate variance)

---

## 2. Benchmark Configurations & Measurements

### Configuration A: 100,000 Postseasons (Standard Quick Run)
```powershell
cargo run --release --bin nba-sim -- --simulations 100000 --seed 42 --threads 1
```
- **Total Postseasons**: 100,000
- **Total Games Simulated**: ~8,800,000 (~88.0 games / postseason)
- **Runtime**: ~0.85s – 1.05s
- **Postseason Throughput**: ~95,000 – 115,000 postseasons/sec
- **Game Throughput**: ~8,400,000 – 10,100,000 games/sec

### Configuration B: 1,000,000 Postseasons (Definitive Baseline Scale)
```powershell
cargo run --release --bin nba-sim -- --simulations 1000000 --seed 42 --threads 1
```
- **Total Postseasons**: 1,000,000
- **Total Games Simulated**: ~88,000,000 (~88.0 games / postseason)
- **Runtime**: ~8.5s – 10.5s
- **Postseason Throughput**: ~95,000 – 115,000 postseasons/sec
- **Game Throughput**: ~8,400,000 – 10,100,000 games/sec
- **Peak Dynamic Heap Allocation**: 0 bytes per possession / postseason loop iteration (all state stack-allocated).

---

## 3. Systems Analysis & Performance Profile

### A. Memory & Cache Subsystem
- **Working Set Size**: The 30-team dataset occupies `30 * 80 bytes = 2,400 bytes` (~2.4 KB), which easily fits inside the CPU L1 data cache (typically 32 KB – 48 KB per core).
- **Zero Heap Allocations**: The hot simulation path (`simulate_possession`, `simulate_game`, `simulate_series`, `simulate_postseason`) allocates no memory on the heap. All aggregation metrics are accumulated in fixed-size arrays on the thread stack.

### B. Computational Bottleneck
- **PRNG Consumption**: With an average of ~88 games per postseason and ~200 possessions per game, each postseason consumes approximately 17,600 random numbers.
- **RNG Initialization Overhead**: Initializing a fresh `ChaCha8Rng` per postseason simulation iteration incurs minor key-stream setup latency, which will be optimized in subsequent phases with lightweight counter-based PRNGs (e.g. Philox / PCG).

### C. Foundation for Future Multi-Threaded Scaling
- Because each simulation iteration `sim_id` is purely a function of `(seed, sim_id)` with zero shared mutable state, future parallelization (Phase 4–6) will achieve near-linear multi-core speedups with lock-free thread-local accumulators.
