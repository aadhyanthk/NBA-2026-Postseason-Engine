# Project Plan: Deterministic High-Throughput NBA 2026 Postseason Simulation Engine

## 0. Project Goal

Transform the existing web-centric stadium/prototype into a serious systems project whose primary artifact is a **native Rust, high-throughput, deterministic, multi-threaded Monte Carlo NBA 2026 Postseason simulation engine** (`nba-sim`).

The simulation engine models the complete NBA 2026 postseason tournament structure:
- **Play-In Tournament** (Eastern & Western Conferences: 7 vs 8, 9 vs 10, Elimination games)
- **NBA Playoffs** (16 teams, 4 rounds of Best-of-7 series with 2-2-1-1-1 home court advantage: First Round, Conference Semifinals, Conference Finals, and NBA Finals)

The React/Tauri application is no longer the core of the project. It becomes an optional telemetry/visualization client.

The final project should demonstrate:

- Strong Rust systems programming
- Multithreading and parallel computation
- Deterministic parallel execution
- Cache-aware data structures
- Data-oriented design (SoA / AoS)
- Performance measurement and profiling
- Benchmark-driven optimization
- Correctness testing and property-based verification
- Understanding of CPU/memory behavior
- Ability to explain engineering tradeoffs rather than merely using libraries

The project must prioritize **correctness, measurement, and defensible engineering decisions over buzzwords**.

Do not implement advanced techniques merely because they sound impressive. Every optimization must have:
1. A concrete performance/correctness motivation.
2. A baseline.
3. A measurement before the change.
4. A measurement after the change.
5. A written explanation of the result.

---

# 1. Starting Point

The current application is:

- React 18 + TypeScript
- Tauri v2
- Ollama / phi3:mini
- Zustand
- Browser JavaScript `setInterval()` at approximately 10 Hz
- Toy weather, gate queue, crowd density, medical, and security simulations
- LLM-driven mock operations

This existing architecture is not suitable as the primary systems artifact.

Do not spend significant effort improving the existing browser simulation.

Instead:

1. Preserve the existing UI if useful.
2. Extract/rebuild the actual simulation engine in Rust.
3. Make the Rust engine independently executable from the command line (`nba-sim`).
4. Make the UI consume telemetry/results from the Rust engine later.

---

# 2. Final Architecture

The desired architecture is:

```text
                         CLI
              seed / threads / simulations
                         |
                         v
            NBA Postseason Configuration
              (Teams, Ratings, Seeds)
                         |
                         v
              Rust Simulation Engine
                         |
          +--------------+--------------+
          |              |              |
          v              v              v
       Worker 1       Worker 2       Worker N
          |              |              |
          +--------------+--------------+
                         |
                         v
                Per-worker statistics
              (Wins, Series, Finals MVP)
                         |
                         v
                    Reduction
                         |
                         v
                 Final statistics
                         |
          +--------------+--------------+
          |                             |
          v                             v
    CLI / benchmark output       Telemetry stream
                                        |
                                        v
                                  Tauri / React
```

The Rust engine must be usable without Tauri or React.

The UI must never be required to run the simulation.

---

# 3. Development Philosophy

## 3.1 Build from simple to advanced

Do NOT begin with:

- Custom work-stealing
- Custom allocators
- SIMD intrinsics
- NUMA tuning
- Lock-free algorithms
- CPU affinity
- Complex cache optimizations

First establish a correct baseline.

The progression should be:

```text
Correct single-threaded simulator
        |
        v
Clean Rust architecture
        |
        v
Deterministic RNG
        |
        v
Parallel simulation
        |
        v
Benchmarking
        |
        v
Data-oriented redesign
        |
        v
Profiling
        |
        v
SIMD / low-level optimization where justified
        |
        v
False-sharing investigation
        |
        v
Custom scheduler / advanced concurrency experiments
```

---

# 4. Phase 0 — [x] Repository Audit

Before changing the project:

1. Inspect the entire existing repository.
2. Identify:
   - Current React architecture
   - Tauri commands
   - Zustand stores
   - Existing simulation logic
   - Existing Ollama integration
   - Existing data models
   - Existing build configuration
3. Determine which parts can be reused.
4. Do not delete working functionality until the replacement architecture is established.

Create:

```text
docs/AUDIT.md
```

Document:

- Current architecture
- Problems with current architecture
- Target architecture
- Migration strategy

At this stage, do not optimize anything.

---

# 5. Phase 1 — [x] Build a Correct Single-Threaded Simulation Engine

## Objective

Create a standalone Rust CLI that can simulate the NBA 2026 Postseason (Play-In + Playoffs) correctly.

The first engine must be intentionally simple.

Do not worry about parallelism yet.

## CLI

Create a command such as:

```bash
cargo run --release -- \
    --seed 42 \
    --simulations 100000
```

Eventually support:

```bash
./nba-sim \
    --seed 42 \
    --simulations 1000000 \
    --threads 8
```

## Initial output

Output statistics such as:

```text
Simulations: 1,000,000
Seed: 42
Format: 2026 NBA Postseason (Play-In + Best-of-7 Playoffs)

Team                  Conf    Play-In %    Playoffs %   Conf Finals %   Finals %    Champion %
Boston Celtics        East    ---          100.0%       68.4%           44.2%       26.8%
Denver Nuggets        West    ---          100.0%       61.2%           39.1%       21.5%
Oklahoma City Thunder West    ---          100.0%       54.8%           32.0%       16.2%
Philadelphia 76ers    East    100.0%        74.5%       28.1%           14.3%        6.1%
...
```

Use integer counters internally where practical.

Calculate percentages only after simulation.

---

# 6. Phase 2 — [x] Build a Defensible Basketball Model

The simulation must not be a meaningless random-number generator.

The model does not need to be academically perfect.

It needs to be:

- Understandable
- Statistically defensible
- Deterministic
- Fast
- Documented

## NBA Postseason Tournament Structure

1. **Play-In Tournament (Per Conference)**:
   - **Game 1**: Seed 7 vs Seed 8 (Winner advances as #7 seed; loser plays in Game 3).
   - **Game 2**: Seed 9 vs Seed 10 (Winner advances to Game 3; loser eliminated).
   - **Game 3**: Loser Game 1 vs Winner Game 2 (Winner advances as #8 seed; loser eliminated).
2. **Playoffs (Best-of-7 Series)**:
   - First Round: 1v8, 2v7, 3v6, 4v5 per conference.
   - Conference Semifinals: Winner 1v8 vs Winner 4v5; Winner 2v7 vs Winner 3v6.
   - Conference Finals: Eastern Conference Finals & Western Conference Finals.
   - NBA Finals: Eastern Champion vs Western Champion.
   - Home Court Schedule (2-2-1-1-1): Games 1, 2, 5, 7 at higher seed; Games 3, 4, 6 at lower seed.
   - Series Winner: First team to win 4 games.

## Basketball Game Simulation Model

Use a defensible possession-based or efficiency-rating model:

- **Possessions per Game ($P$)**: Baseline league pace (~100 possessions/game), modified by team pace ratings.
- **Offensive & Defensive Ratings ($ORtg$, $DRtg$)**: Points scored/allowed per 100 possessions.
- **Home Court Advantage ($HCA$)**: ~+3.0 to +3.5 points per 100 possessions added to home team efficiency.
- **Expected Points per Possession**:
  $$\mu_{\text{home}} = \frac{ORtg_{\text{home}} + DRtg_{\text{away}} - LeagueAvg}{100} + HCA$$
  $$\mu_{\text{away}} = \frac{ORtg_{\text{away}} + DRtg_{\text{home}} - LeagueAvg}{100}$$
- **Score Generation**: Normal distribution or discrete possession Markov sampling (2PT, 3PT, FT, Turnovers).
- **Overtime Resolution**: If regulation score is tied (score_home == score_away), simulate 5-minute OT periods (approx. 10 possessions each) until a winner is determined (no ties in basketball).

Create:

```text
docs/MODEL.md
```

Explain:

- What the model assumes (Possession pace, offensive/defensive ratings, Four Factors)
- Why possession-based efficiency modeling is standard for basketball
- How team ratings and pace interact
- How home court advantage (2-2-1-1-1) affects series win probability
- How overtime resolution works
- Limitations of the model
- Potential future improvements

The purpose is to provide a credible computational workload.

---

# 7. Phase 3 — [x] Establish the Baseline

Before introducing advanced optimization, benchmark the basic implementation.

Measure:

- Total runtime
- Simulations/second (complete postseasons/sec)
- Matches/second (individual basketball games/sec)
- CPU utilization if available
- Memory usage

Create a reproducible benchmark command.

Example:

```bash
./nba-sim \
    --seed 42 \
    --simulations 1000000 \
    --threads 1
```

Record the result.

Create:

```text
benchmarks/baseline.md
```

Example:

```text
Machine:
CPU:
RAM:
OS:
Rust version:
Compiler flags:

1,000,000 simulations (complete postseasons)

Runtime:
Throughput (postseasons/sec):
Throughput (games/sec):
```

Never replace the baseline measurement.

Keep historical results so improvements can be demonstrated.

---

# 8. Phase 4 — [x] Understand and Implement Deterministic Randomness

This is one of the most important project features.

## Problem

A naive shared RNG can produce different results depending on thread execution order.

For example:

```text
threads = 1
```

may produce one random sequence while:

```text
threads = 8
```

produces a different sequence.

This makes debugging and reproducibility impossible across thread counts.

## Required property

The following must produce bit-for-bit identical results:

```bash
./nba-sim --seed 42 --threads 1
./nba-sim --seed 42 --threads 2
./nba-sim --seed 42 --threads 4
./nba-sim --seed 42 --threads 8
./nba-sim --seed 42 --threads 16
```

The result must not depend on which thread executes which simulation.

## Design

Use a counter-based or otherwise splittable deterministic PRNG.

The conceptual API should be:

```text
random(
    global_seed,
    simulation_id,
    series_or_game_id,
    random_draw_id
)
```

For example:

```text
random(42, 1837, 5, 24)
```

always produces the same value.

Possible PRNGs to investigate:

- Philox4x32
- PCG64 variants
- ChaCha8 / SplitMix64 / counter-based generators

Do not choose a generator solely because its name sounds sophisticated.

Document the choice in:

```text
docs/RNG.md
```

Explain:

- What a PRNG is
- What a seed is
- Why a shared sequential RNG is problematic in parallel code
- Why simulation-indexed randomness solves the scheduling problem
- Why the chosen generator was selected
- What exactly "deterministic" means in this project

---

# 9. Phase 5 — [x] Determinism Verification

Build a dedicated verification mode.

Example:

```bash
./nba-sim verify-determinism \
    --seed 42 \
    --simulations 1000000
```

It should execute the same workload using multiple thread counts.

Example:

```text
threads=1
threads=2
threads=4
threads=8
threads=16
threads=32
```

Then compare the serialized final results.

Use SHA-256 as an additional verification mechanism.

Example:

```text
threads=1   SHA256: 7f83b1657ff1...
threads=2   SHA256: 7f83b1657ff1...
threads=4   SHA256: 7f83b1657ff1...
threads=8   SHA256: 7f83b1657ff1...
threads=16  SHA256: 7f83b1657ff1...
threads=32  SHA256: 7f83b1657ff1...

PASS: all outputs identical
```

Important:

SHA-256 is not what makes the program deterministic.

It is merely a convenient way to verify that the resulting bytes are identical.

---

# 10. Phase 6 â€” Correctness Testing

Before optimizing, establish strong tests.

## Unit tests

Test:

- Team initialization & rating loading
- Game simulation (regulation scoring & overtime handling)
- Play-In tournament bracket progression
- Best-of-7 series 2-2-1-1-1 home court alternation & win thresholds (first to 4 wins)
- Conference bracket progression & Finals matchup
- Statistics accumulation
- Serialization
- RNG counter behavior

## Invariants

Examples:

```text
Total Champions = number of simulated postseasons
Total Play-In Qualifiers = 4 per conference (seeds 7 and 8 decided)
Total Playoff Series per Postseason = 15 series (14 best-of-7 + 6 play-in games)
No negative counters
All team identifiers are valid
Probabilities are within [0, 1]
Probabilities sum to exactly 1.0 (or 100%)
```

## Property-based testing

Investigate `proptest`.

Use it for properties such as:

```text
For any valid team configuration:
    simulation never produces invalid team IDs
```

and:

```text
For any valid postseason run:
    exactly one NBA Champion exists
    champion won exactly 4 games in the Finals
```

Do not create meaningless property tests just to use the library.

---

# 11. Phase 7 — [x] Parallelize the Simulation

Now introduce multithreading.

Start with Rayon.

Why Rayon?

Because the primary goal is to learn and demonstrate parallel computation, not to reinvent a mature scheduler immediately.

Conceptually:

```text
10,000,000 postseasons
        |
        +---- Worker 1
        +---- Worker 2
        +---- Worker 3
        ...
        +---- Worker N
```

Use independent simulations so that most work does not require synchronization.

Start with:

```text
1 thread
2 threads
4 threads
8 threads
16 threads
```

Add 32 only if the test machine has enough logical/physical resources to make that measurement meaningful.

---

# 12. Phase 8 — [x] Avoid Shared Mutable State

Do NOT have every worker update a global counter if it can be avoided.

Bad conceptual design:

```text
Thread 1 ----\
Thread 2 -----\
Thread 3 ------> shared counter (Atomic / Mutex lock contention)
Thread 4 -----/
...
```

This creates severe contention and cache-line bouncing.

Prefer:

```text
Thread 1 -> local statistics
Thread 2 -> local statistics
Thread 3 -> local statistics
Thread 4 -> local statistics

                |
             reduction
                |
          final statistics
```

This teaches an important systems principle:

> Avoid synchronization by designing ownership and data flow correctly.

Use thread-local/per-worker accumulators.

---

# 13. Phase 9 — [x] Reduction

Combine per-thread results into one final result.

Initial implementation:

```text
local_result_1
local_result_2
local_result_3
...
        |
combine (Tree Reduction)
        |
final_result
```

Do not immediately implement a custom lock-free reduction.

First establish correctness and benchmark the straightforward design.

A tree-style reduction is acceptable.

Document:

```text
docs/REDUCTION.md
```

Explain:

- Why shared global counters can become a bottleneck
- What a reduction is
- Why local accumulators reduce contention
- Whether floating-point order can affect reproducibility (use integer counters!)
- Why integer counters are essential for bit-identical tournament statistics

---

# 14. Phase 10 — [x] Parallel Scaling Benchmarks

Now create a proper scaling benchmark.

Run:

```text
1 thread
2 threads
4 threads
8 threads
16 threads
32 threads
```

Measure:

- Runtime
- Throughput (postseasons/sec and games/sec)
- Speedup
- Parallel efficiency

Calculate:

```text
speedup(N) = runtime(1 thread) / runtime(N threads)

efficiency(N) = speedup(N) / N
```

Create graphs.

Example:

```text
Threads    Runtime    Speedup    Efficiency
1          10.0 s     1.0x       100%
2           5.3 s     1.89x       94%
4           2.7 s     3.70x       92%
8           1.5 s     6.67x       83%
16          0.9 s    11.11x       69%
32          0.7 s    14.29x       45%
```

The numbers above are examples only.

Never fabricate benchmark results.

---

# 15. Phase 11 — [x] Learn and Apply Amdahl's Law

Analyze why scaling is not perfectly linear.

Amdahl's Law:

```text
speedup(N) = 1 / (S + (1 - S) / N)
```

where:

```text
S = fraction of work that is serial (initialization, thread spawning, reduction, I/O)
```

Use this to reason about the measured scaling curve.

The README should explicitly discuss:

- Why 2 threads aren't necessarily 2x faster
- Why 32 threads aren't necessarily 32x faster
- Serialization overhead
- Scheduling overhead
- Reduction overhead
- Memory-system / cache bandwidth limitations
- Other bottlenecks discovered during profiling

---

# 16. Phase 12 — [x] Data-Oriented Design

Once the parallel baseline works, investigate the data layout.

Understand:

- Array of Structures (AoS)
- Structure of Arrays (SoA)
- CPU cache (L1d, L2, L3)
- Cache lines (64 bytes)
- Spatial locality & Temporal locality

## AoS (Array of Structures)

Conceptually:

```rust
struct Game {
    home_team: u8,
    away_team: u8,
    home_score: u16,
    away_score: u16,
    winner: u8,
}
```

Stored in memory as contiguous array of `Game` structs.

## SoA (Structure of Arrays)

Conceptually:

```rust
struct GameBatch {
    home_teams: Vec<u8>,
    away_teams: Vec<u8>,
    home_scores: Vec<u16>,
    away_scores: Vec<u16>,
    winners: Vec<u8>,
}
```

Choose SoA only where the access pattern benefits from contiguous vector loads and cache-line density.

Do not convert every structure to SoA automatically.

---

# 17. Phase 13 — [x] Cache-Aware Benchmarking

Create a controlled comparison:

```text
AoS implementation
vs
SoA implementation
```

Measure:

- Runtime
- Throughput
- Cache misses (L1d / LLC) via `perf` or platform counters if tooling permits
- CPU utilization

Explain whether the change actually helped.

If SoA does not help a particular component, document why.

That is a valid engineering result.

---

# 18. Phase 14 — [x] Allocation Analysis

Inspect allocations in the simulation hot path.

The goal is:

> Zero heap allocations per simulated postseason / game in the hot loop.

Prefer:

- Stack / local registers
- Reused preallocated buffers
- Fixed-size arrays (`[u8; N]`, `[u32; 30]`)
- Preallocated vectors outside the loop
- Thread-local scratch arenas

Do not introduce a custom allocator unless profiling demonstrates an allocation bottleneck.

The goal is not "zero allocations everywhere in the binary."

The goal is:

> Avoid heap allocations (`malloc`, `free`, `Vec::push`, `Box`) in the inner simulation loop.

Document allocation behavior in `docs/MEMORY.md`.

---

# 19. Phase 15 — [x] Profiling

Learn to profile before attempting further optimization.

Potential tools:

- `cargo flamegraph`
- Linux `perf`
- Instruments on macOS
- Windows Performance Analyzer / VTune / platform-appropriate profilers

Determine where runtime is actually going.

Example:

```text
simulate_game        48%
random generation    31%
series_logic         11%
reduction / stats     6%
other                 4%
```

These numbers are illustrative only.

Use actual measurements.

Create:

```text
docs/PROFILING.md
```

For each major optimization, record:

```text
Hypothesis:
Measurement:
Change:
Result:
Explanation:
```

---

# 20. Phase 16 — [x] SIMD

Only after profiling identifies suitable arithmetic-heavy loops should SIMD be investigated.

SIMD = Single Instruction, Multiple Data (e.g. AVX-512, AVX2, NEON).

Conceptually:

```text
[score_1, score_2, score_3, score_4]
+
[delta_1, delta_2, delta_3, delta_4]
=
[res_1,   res_2,   res_3,   res_4]
```

Investigate whether the compiler automatically vectorizes the hot loop first (`RUSTFLAGS="-C target-cpu=native"`).

Only use explicit SIMD if it produces a measurable benefit.

Potential approaches:

- Compiler auto-vectorization
- `std::simd` / `portable_simd` (nightly) or `wide` / `packed_simd`
- Architecture-specific intrinsics only when justified

Benchmark:

```text
scalar baseline
vs
compiler-vectorized
vs
explicit SIMD
```

Do not sacrifice portability or maintainability for a tiny benchmark gain.

---

# 21. Phase 17 — [x] Branch Prediction

Identify branches inside extremely hot loops.

Understand:

- Conditional branches (`if/else`, `match`)
- Predictable branches (e.g. Series check `wins < 4`)
- Unpredictable branches (e.g. coin-flip game outcomes)
- Branch misprediction penalty (~15â€“20 CPU cycles on modern x86_64)

Only optimize branches if profiling/benchmarking suggests branch mispredictions matter.

Potential investigation:

```text
branch-heavy game logic
vs
branchless / arithmetic selection (e.g. cmov, bitmasking)
```

Measure actual performance.

Do not blindly remove branches.

---

# 22. Phase 18 — [x] False Sharing

Understand cache lines.

A typical cache line is 64 bytes.

False sharing occurs when:

```text
Thread A modifies variable A
Thread B modifies variable B
```

but both variables happen to reside on the same 64-byte cache line.

This creates expensive cache-coherence traffic (MESI/MOESI protocol invalidations across CPU cores).

## Investigation

Create a benchmark:

```text
adjacent per-thread accumulators in a single array (unpadded)
vs
cache-line-separated accumulators (padded with #[repr(align(64))])
```

Document:

- What false sharing is
- Why it occurs
- How it was detected
- What changed (`#[repr(align(64))]`)
- Whether multi-threaded scaling improved

---

# 23. Phase 19 — [x] Custom Work-Stealing Scheduler (Optional Advanced Phase)

Only attempt this after the Rayon implementation is complete and benchmarked.

Goal:

> Build a simplified work-stealing scheduler as an experimental comparison against Rayon.

Study:

- Worker threads
- Work queues (Chase-Lev deque)
- Task ownership
- Work stealing
- Atomics & Memory Ordering (`Acquire`/`Release`/`Relaxed`)
- Scheduling overhead vs morsel batch size

Possible tools/crates to investigate:

- `crossbeam-deque`

The custom scheduler is NOT intended to replace Rayon in the production version.

Instead, compare:

```text
Rayon
vs
custom work-stealing scheduler
```

Measure:

- Throughput
- Scaling curve
- Scheduling overhead
- Complexity & reliability

A successful outcome may be:

> "My scheduler achieved 88% of Rayon's throughput, and profiling showed that task allocation chunk size was the difference."

---

# 24. Phase 20 â€” NUMA and CPU Affinity (Optional)

Only investigate this if the benchmark environment supports meaningful multi-socket or high-core-count testing.

Understand:

- NUMA (Non-Uniform Memory Access)
- Local vs Remote memory latency
- CPU core pinning / affinity
- Thread migration penalties
- First-touch memory allocation

Potential experiment:

```text
default OS scheduling
vs
CPU-pinned workers
```

---

# 25. Phase 21 â€” Benchmark Suite

Create a proper benchmark suite.

Recommended categories:

## Correctness

```text
determinism across threads (1..32)
tournament invariants
property tests
```

## Single-thread performance

```text
baseline
SoA vs AoS
zero-allocation hot loop
vectorization
```

## Parallel performance

```text
1 thread
2 threads
4 threads
8 threads
16 threads
32 threads
```

## Concurrency experiments

```text
shared atomic accumulator
vs
padded thread-local accumulator (#[repr(align(64))])
```

## Scheduler experiments

```text
Rayon
vs
custom work-stealing scheduler
```

---

# 26. Benchmark Methodology

Every benchmark must document:

- CPU model & microarchitecture
- Physical cores & Logical threads
- L1d, L2, and L3 cache sizes
- RAM speed & channels
- OS version
- Rust & LLVM version
- Compiler optimization flags (`-C opt-level=3`, `target-cpu=native`)
- Number of simulations (e.g. 10M postseasons)
- Master Seed
- Thread count
- Number of iterations & variance / standard deviation
- Machine background load status

Avoid comparing numbers from completely different machines as if they were directly equivalent.

Always use `--release` builds.

---

# 27. Determinism CI

Add automated CI (GitHub Actions).

Every commit should verify that:

```text
seed = 42
```

produces bit-for-bit identical outputs across supported thread counts (`1`, `2`, `4`, `8`).

A local verification command tests `16` and `32` threads.

---

# 28. Repository Structure

Aim for something approximately like:

```text
nba-2026-postseason-engine/
â”œâ”€â”€ Cargo.toml
â”œâ”€â”€ benches/
â”‚   â”œâ”€â”€ scaling_benchmark.rs
â”‚   â””â”€â”€ memory_benchmark.rs
â”œâ”€â”€ docs/
â”‚   â”œâ”€â”€ AUDIT.md
â”‚   â”œâ”€â”€ MODEL.md
â”‚   â”œâ”€â”€ RNG.md
â”‚   â”œâ”€â”€ REDUCTION.md
â”‚   â”œâ”€â”€ MEMORY.md
â”‚   â””â”€â”€ PROFILING.md
â”œâ”€â”€ src/
â”‚   â”œâ”€â”€ bin/
â”‚   â”‚   â””â”€â”€ nba-sim.rs          # Standalone CLI binary
â”‚   â”œâ”€â”€ core/
â”‚   â”‚   â”œâ”€â”€ types.rs            # Bit-packed teams, series, game states
â”‚   â”‚   â”œâ”€â”€ teams.rs            # NBA team ratings (Off/Def efficiency, pace)
â”‚   â”‚   â””â”€â”€ rng.rs              # Hierarchical counter-based PRNG
â”‚   â”œâ”€â”€ sim/
â”‚   â”‚   â”œâ”€â”€ game.rs             # Fast basketball game simulation
â”‚   â”‚   â”œâ”€â”€ play_in.rs          # 7v8, 9v10, elimination play-in bracket
â”‚   â”‚   â”œâ”€â”€ playoffs.rs         # Best-of-7 series & bracket progression
â”‚   â”‚   â””â”€â”€ postseason.rs       # Complete postseason tournament
â”‚   â”œâ”€â”€ parallel/
â”‚   â”‚   â”œâ”€â”€ worker.rs           # Thread execution & batch chunking
â”‚   â”‚   â”œâ”€â”€ accumulator.rs      # Cache-padded per-thread stats (#[repr(align(64))])
â”‚   â”‚   â””â”€â”€ reduction.rs        # Deterministic tree reduction
â”‚   â”œâ”€â”€ lib.rs
â”‚   â””â”€â”€ main.rs
â”œâ”€â”€ tests/
â”‚   â”œâ”€â”€ correctness.rs
â”‚   â”œâ”€â”€ determinism.rs
â”‚   â””â”€â”€ invariants.rs
â””â”€â”€ README.md
```

---

# 29. Tauri / React UI

Only work on the UI after the engine is stable, correct, deterministic, and benchmarked.

The UI is strictly an optional visualization and telemetry monitor:
- Displays live Monte Carlo tournament probability heatmaps
- Displays series bracket win likelihoods
- Visualizes CPU thread utilization, matches/sec throughput meter, and determinism verification state

---

# 30. Telemetry

If the UI is retained, expose useful engine telemetry:
- Real-time postseasons simulated / sec
- Active worker thread count
- L1/L2 cache hit rate / memory bandwidth
- Seed & determinism hash confirmation

---

# 31. Performance Claims

Never put an unverified claim in the README.

Do not write: "Ultra-fast lock-free engine" unless lock-free reduction is measured, benchmarked, and verified against alternatives.

Write:
> "Simulates 10,000,000 complete NBA postseasons (approx. 850,000,000 basketball games) in 1.42 seconds on an AMD Ryzen 9 7950X (32 threads), achieving a 26.4x speedup over single-threaded baseline."

---

# 32. "Unimpeachable" Evidence Checklist

The final repository should contain evidence for:

1. **Deterministic Parallel Execution**: Bitwise identical outputs across 1 to 32 threads (`verify-determinism` SHA-256 match).
2. **Multi-Threaded Scaling**: Amdahl's Law curve with measured speedup and efficiency data.
3. **Data-Oriented Layout**: Documented SoA vs AoS memory layout and cache miss comparison.
4. **Zero-Allocation Hot Path**: Memory analysis showing 0 heap allocations per game in simulation loop.
5. **Cache-Line Alignment**: Padded thread accumulators (`#[repr(align(64))]`) preventing false sharing.
6. **Correct Tournament Rules**: 100% compliant with 2026 NBA Play-In & Playoff Best-of-7 series mechanics.

---

# 33. What NOT to Claim

Do not claim:
- "Predicts the exact winner of the 2026 NBA Championship." (It is a probabilistic Monte Carlo computational model).
- "Zero latency / lock-free magic" without profiling proof.
- "Novel AI simulation" when using Markov/Poisson mathematical models.

---

# 34. README Structure

The final README should contain:
1. Executive Summary & Architecture Diagram
2. Systems Highlights (Determinism, Data Layout, Concurrency, Zero-Alloc Hot Path)
3. NBA 2026 Postseason Model & Mechanics
4. Empirical Benchmark Results (Throughput, Scaling Curves, Amdahl Analysis)
5. CLI Usage & Reproducibility Guide (`--simulations`, `--threads`, `--seed`, `verify-determinism`)
6. Systems Interview Defense FAQ

---

# 35. Resume Positioning

**Example Resume Bullet:**
> "Engineered a high-throughput, deterministic NBA postseason Monte Carlo simulation engine in Rust; achieved bit-identical results across 1â€“32 threads via hierarchical counter-based PRNG stream splitting, and simulated 10M+ full tournaments in <1.5s using cache-conscious SoA data structures and padded lock-free accumulators."

---

# 36. Interview Preparation Requirements

The project is complete when you can defend it under grilling:

### CPU / Memory
- Cache lines (64B), L1/L2/L3 hierarchy, spatial vs temporal locality
- AoS vs SoA cache utilization
- Branch prediction penalty & branchless selection
- SIMD auto-vectorization

### Concurrency & Parallelism
- Data races vs race conditions
- Lock contention & cache-line bouncing (False Sharing)
- Why `#[repr(align(64))]` fixes false sharing
- Amdahl's Law & scaling limits

### Randomness & Determinism
- Counter-based PRNGs (Philox4x32/PCG64) vs global state PRNGs
- Why stream splitting guarantees deterministic parallel execution regardless of thread scheduling

### Basketball Domain Model
- Four Factors, pace ratings, home court advantage (2-2-1-1-1), overtime resolution

---

# 37. Suggested Learning Strategy

Learn concepts immediately before implementing them:
```text
Learn thread parallelism
    |
Implement basic threaded simulation (Rayon)
    |
Measure baseline & scaling
    |
Learn false sharing & cache lines
    |
Padded thread accumulators (#[repr(align(64))])
    |
Measure speedup
    |
Learn counter-based PRNGs
    |
Implement stream splitting & determinism verification
    |
Verify bitwise reproducibility
```

---

# 38. Milestone Definitions

- **Milestone 1 â€” Engine exists**: Standalone Rust CLI, correct NBA Play-In & Best-of-7 Playoff tournament simulation, tests, seeded single-threaded execution.
- **Milestone 2 â€” Deterministic**: Counter-based PRNG, identical output across any thread count, `verify-determinism` SHA-256 test.
- **Milestone 3 â€” Parallel**: Rayon parallel execution, 1..32 thread benchmarks, thread-local accumulators, tree reduction, Amdahl curve.
- **Milestone 4 â€” Performance & Memory**: Benchmark harness, profiling, zero-allocation hot loop, SoA vs AoS experiment.
- **Milestone 5 â€” Cache & Concurrency Hardening**: False sharing investigation, `#[repr(align(64))]` alignment, branch prediction analysis.
- **Milestone 6 â€” Optional Advanced Systems**: Custom work-stealing deque scheduler comparison, NUMA analysis.
- **Milestone 7 â€” Presentation & Docs**: Technical whitepaper, README, benchmark charts, interview defense walkthrough.

---

# 39. Priority Order

```text
1. Correct NBA simulation (Play-In + Best-of-7 Playoffs)
2. Clean Rust architecture & types
3. Deterministic counter-based PRNG
4. Parallel execution & thread-local reduction
5. Benchmarking & Amdahl scaling analysis
6. Profiling & zero-allocation hot path
7. SoA / cache layout optimization
8. False-sharing elimination (#[repr(align(64))])
9. Branch / SIMD analysis
10. Optional custom scheduler
11. UI telemetry bridge
```

---

# 40. Definition of Done

- The engine runs independently from the CLI (`nba-sim`).
- NBA Play-In and Best-of-7 Playoff mechanics are 100% correct.
- Strict determinism is proven across thread counts (`1` to `32`).
- Multi-threaded scaling is measured and documented with Amdahl analysis.
- Hot loop has zero dynamic heap allocations.
- False sharing is eliminated with cache-line padded accumulators.
- Profiling and benchmark data are documented in `docs/` and `benchmarks/`.

---

# 41. Instructions for Antigravity / Coding Agent

1. **Inspect before modifying.**
2. Maintain clear separation between the Rust engine and UI.
3. Do not rewrite the entire repository in one operation.
4. Implement one milestone at a time.
5. Run tests after every meaningful architectural change.
6. Run benchmarks before and after performance changes.
7. Never fabricate benchmark results.
8. Every optimization must have a documented systems motivation.
9. Keep commits logically separated by milestone.
10. Treat benchmark evidence as more authoritative than assumptions.

---

# 42. First Task for the Agent

Do NOT begin by implementing the entire target architecture.

First:
1. Inspect the repository.
2. Create `docs/ARCHITECTURE.md` establishing the NBA 2026 Postseason Engine design.
3. Implement **Milestone 1 only**:
   > A correct, standalone, single-threaded Rust NBA tournament simulation engine (`nba-sim`) with a seeded CLI, correct Play-In & Best-of-7 playoff progression, tests, and basic documentation.
