# Basketball Simulation Engine — Resume Integration Instructions for Antigravity

## Purpose

Integrate the hardened **Basketball Simulation Engine** into the three resume variants:

1. `AadhyanthK_Resume_SWE(2).pdf` → SWE / systems / backend / infrastructure
2. `AadhyanthK_Resume_Quant(1).pdf` → quantitative engineering / quant research / trading / performance-oriented roles
3. `AadhyanthK_Resume_Cybersecurity.pdf` → cybersecurity / security engineering / low-level systems

The project is intended to replace the current **FIFA-26 Operations Control Center** project once the new native engine is genuinely implemented and benchmarked.

Do **not** simply add another project. The current resumes are already dense. The goal is to make this one project materially stronger than the current FIFA project while preserving the strongest role-specific projects.

---

# 1. Core project identity

## Recommended canonical title

**Deterministic Parallel Basketball Simulation Engine**

Alternative if the implementation proves the performance claim:

**High-Throughput Deterministic Basketball Simulation Engine**

Do not use "High-Throughput" merely because the architecture is intended to be high-throughput. It should be supported by measured benchmark results.

## One-sentence identity

A native Rust Monte Carlo basketball simulation engine designed to execute large numbers of independent possession-level simulations deterministically across multiple CPU cores, with reproducible RNG, parallel scheduling, profiling, and benchmark-driven optimization.

The important distinction is:

> Basketball is the workload. Systems engineering is the project.

Do not present this as a sports app, fantasy-basketball dashboard, or visualization project.

---

# 2. Resume-specific positioning

## A. SWE / Systems resume

Current SWE resume:
- Skills emphasize C++/C/Python/Java/TypeScript, DSA, OOD, system design, concurrency, memory management, POSIX/Linux, REST/WebSockets.
- Current featured projects include FIFA Operations Control Center, Concurrency Control Simulator, PCA simulator, and RSA messaging.
- The FIFA project currently reads primarily as a Tauri/React/local-LLM application.

The new project should replace that project because it gives substantially stronger evidence of:
- Rust
- systems programming
- concurrency
- deterministic execution
- parallel computation
- memory-conscious design
- benchmarking
- profiling
- performance engineering
- Linux tooling
- reproducible computation

### SWE project header

**Deterministic Parallel Basketball Simulation Engine | Rust, Rayon, SIMD, Criterion, Linux**

Only list technologies that actually exist in the final implementation.

If SIMD is not implemented and benchmarked, remove `SIMD`.

If Rayon is replaced by a justified custom scheduler, change the header accordingly.

### Recommended SWE bullets

Use 3 bullets.

**Bullet 1 — architecture**
> Engineered a native Rust Monte Carlo basketball simulation engine using a possession-level Markov model and data-oriented state representation to execute [N] independent simulations with reproducible results.

Replace `[N]` with a measured workload.

**Bullet 2 — parallelism/determinism**
> Parallelized independent simulations across [N] worker threads with deterministic counter/keyed PRNG streams, local accumulators, and reduction-based aggregation, producing bit-identical outputs across thread counts.

Use this only after the cross-thread determinism test actually passes.

**Bullet 3 — performance**
> Benchmarked and profiled [baseline → optimized] execution using Criterion and Linux performance counters, improving throughput by [X]× while measuring [IPC/cache misses/branch misses/etc.] to identify bottlenecks.

Do not claim every metric. Include only metrics actually collected.

### SWE technology keywords to prioritize

Strong:
- Rust
- Rayon
- multithreading
- parallel computing
- Monte Carlo
- deterministic execution
- PRNG
- data-oriented design
- memory layout
- Criterion
- Linux perf
- profiling
- benchmarking
- SIMD, if real

Lower priority:
- React
- Tauri
- TypeScript
- charts
- dashboard
- UI

The UI is a telemetry visualizer. It is not the selling point.

---

# 3. Quant resume

Current Quant resume:
- Skills emphasize Modern C++, C, Python, probability/statistics, Monte Carlo uncertainty, multithreading/concurrency, memory layout, Linux systems, linear algebra, latency optimization.
- Current featured projects include the concurrency simulator, PCA/eigen-decomposition simulator, and saliency-guided compression.
- This resume already has unusually good alignment with the new project.

The basketball engine should become the flagship quantitative/systems project.

### Quant project header

**Deterministic Parallel Basketball Simulation Engine | Rust, Monte Carlo, Markov Models, Multithreading**

Potential additions:
- Statistical Modeling
- Criterion
- SIMD
- Linux perf

Only include them if actually implemented.

### Recommended Quant bullets

Use 3 bullets.

**Bullet 1 — mathematical model**
> Built a possession-level Markov Monte Carlo model of basketball games, modeling state-dependent transitions including shot attempts, turnovers, rebounds, fouls, free throws, and possession changes to estimate tournament outcomes.

Only mention event classes actually implemented.

**Bullet 2 — large-scale simulation**
> Implemented deterministic parallel Monte Carlo execution across [N] CPU threads using independent keyed PRNG streams and local aggregation, enabling [N] simulations in [T] while preserving identical results across thread configurations.

The `[N] simulations in [T]` claim must come directly from a benchmark.

**Bullet 3 — performance analysis**
> Profiled scaling and bottlenecks with Criterion/Linux performance counters, quantifying [X]× parallel speedup and [Y]% efficiency while analyzing [IPC/cache/branch/memory] behavior.

Again: measured values only.

### Quant framing

The project should communicate:

**probabilistic model → Monte Carlo workload → deterministic computation → parallel execution → empirical performance analysis**

It should NOT communicate:

**basketball UI → sports visualization → app development**

Do not waste a bullet on the Tauri interface.

---

# 4. Cybersecurity resume

Current Cybersecurity resume:
- Skills emphasize application security, cryptographic protocols, hardware/RF security, reverse engineering, threat hunting, network analysis, memory safety/exploits.
- Security research is the strongest part of this resume.
- Current projects include FIFA Operations Control Center, RSA messaging, Concurrency Control Simulator, and embedded cryptographic engine.

The basketball engine is NOT a cybersecurity project.

Do not pretend that it is.

Its value to the security resume is that it demonstrates stronger low-level/software-engineering ability and complements the existing security research.

### Cyber project header

**Deterministic Parallel Basketball Simulation Engine | Rust, Systems Programming, Concurrency, Linux**

Do not put cybersecurity buzzwords into the title.

### Recommended Cyber bullets

Use 2–3 bullets depending on available space.

**Bullet 1 — low-level systems**
> Engineered a native Rust simulation engine with explicit state representation, deterministic execution, and parallel worker execution for large-scale Monte Carlo basketball simulations.

**Bullet 2 — correctness/reproducibility**
> Designed keyed deterministic PRNG streams and cross-thread reproducibility tests, verifying bit-identical simulation outputs independent of worker count.

**Bullet 3 — systems analysis**
> Profiled memory behavior, CPU utilization, synchronization overhead, and execution scaling under Linux to identify and eliminate performance bottlenecks.

Only retain the third bullet if this analysis actually exists.

### Cyber resume priority

The project should remain below:
1. Automotive Keyless Entry Protocols & Cryptographic Security
2. Strongest security-specific projects

Do not let the basketball project displace the automotive security research.

Its role is to show:

> "This candidate is not only a security student; they can also build and reason about serious native systems."

---

# 5. What should happen to the existing FIFA project

The current resumes describe:

- Tauri + React
- local LLM / Ollama
- structured JSON tool calling
- network isolation
- Zustand
- 10 Hz deterministic simulation
- 60 FPS rendering
- testing

These details are visible in the current SWE and Cyber resumes.

The new engine should be treated as a **major technical rewrite / successor**, not as a cosmetic rename.

Do NOT leave the old claims attached to the new project unless they remain true.

If the Tauri/React telemetry dashboard remains, describe it as a thin visualization/control layer around the Rust engine.

For example:

**Deterministic Parallel Basketball Simulation Engine | Rust, Rayon, Tauri, React, Criterion**

But only include Tauri/React if they are still part of the delivered project.

---

# 6. What NOT to claim

Antigravity must never fabricate or prematurely add any of these:

- "10M simulations/sec"
- "million games in seconds"
- "lock-free"
- "zero-copy"
- "zero allocation"
- "cache optimized"
- "cache-line optimized"
- "NUMA-aware"
- "SIMD accelerated"
- "AVX2"
- "AVX-512"
- "work stealing"
- "custom scheduler"
- "linear scaling"
- "near-linear scaling"
- "real-time"
- "production-grade"
- "HPC-grade"

unless the implementation and measurement support the claim.

Architecture alone is not evidence.

---

# 7. Performance numbers must be generated, not invented

Before inserting a number into any resume bullet, Antigravity should be able to point to:

1. the benchmark command,
2. the benchmark output,
3. the benchmark configuration,
4. the commit/version producing the result,
5. the machine/environment,
6. the baseline being compared against.

Examples:

- simulations/sec
- games/sec
- execution time
- single-thread runtime
- 2/4/8/16/32-thread runtime
- speedup
- parallel efficiency
- memory usage
- allocation count
- IPC
- cache misses
- branch mispredictions

If the result is not reproducible, do not put it on the resume.

---

# 8. Determinism is a major resume differentiator

This should be one of the project's strongest features.

Target behavior:

```text
seed = 42
simulations = N

threads = 1   -> digest A
threads = 2   -> digest A
threads = 4   -> digest A
threads = 8   -> digest A
threads = 16  -> digest A
threads = 32  -> digest A
```

The digest should remain identical if the simulation semantics and configuration are identical.

Recommended implementation:

- deterministic seed
- simulation ID
- event/draw ID
- keyed/counter-based PRNG
- no dependence on scheduling order
- deterministic reduction/aggregation where required
- SHA-256 verification output

Do not use a shared mutable RNG across worker threads.

Do not rely on thread-local RNG initialization if it makes results depend on scheduling.

---

# 9. Recommended benchmark matrix

Antigravity should generate a reproducible benchmark matrix such as:

| Threads | Simulations | Runtime | Sims/sec | Speedup | Efficiency |
|---:|---:|---:|---:|---:|---:|
| 1 | N | T1 | X1 | 1.00× | 100% |
| 2 | N | T2 | X2 | S2 | E2 |
| 4 | N | T4 | X4 | S4 | E4 |
| 8 | N | T8 | X8 | S8 | E8 |
| 16 | N | T16 | X16 | S16 | E16 |
| 32 | N | T32 | X32 | S32 | E32 |

Use the actual CPU's available thread count.

Do not manufacture a 32-thread benchmark on a machine that does not have 32 useful hardware threads.

---

# 10. Baseline versus optimization

The project needs a defensible performance story.

Recommended progression:

### Baseline
- correct single-threaded Rust implementation
- straightforward data structures
- deterministic simulation
- benchmark

### Parallel baseline
- Rayon parallel execution
- deterministic per-simulation RNG
- local accumulation
- reduction

### Data/layout optimization
Only if profiling indicates a benefit:
- SoA
- tighter state representation
- reduced pointer chasing
- reduced allocations

### CPU optimization
Only if justified:
- SIMD
- branch reduction
- cache-aware layout
- specialized hot loops

### Scheduler optimization
Only if profiling shows Rayon is insufficient:
- custom work-stealing scheduler
- specialized task granularity
- affinity experiments

The resume should describe the measured result of this progression, not the intended progression.

---

# 11. Project ordering by resume

## SWE

Recommended:

1. Deterministic Parallel Basketball Simulation Engine
2. Concurrency Control Simulator & Deadlock Analyzer
3. Interactive PCA & Matrix Decomposition Simulator
4. RSA Encrypted Messaging & Cryptosystem

The new engine should be #1 because it is the strongest systems signal.

## Quant

Recommended:

1. Deterministic Parallel Basketball Simulation Engine
2. Concurrency Control Simulator & Deadlock Analyzer
3. Multi-Task Deep Learning research
4. PCA / Eigen-Decomposition or Saliency Compression, depending on final resume space

The exact order can depend on the role, but the basketball engine should be near the top.

## Cybersecurity

Recommended:

1. Automotive Keyless Entry Protocols & Cryptographic Security
2. RSA Encrypted Messaging & Cryptosystem
3. Deterministic Parallel Basketball Simulation Engine
4. Concurrency Control Simulator & Deadlock Analyzer
5. Embedded Cryptographic Engine if space permits

The new engine should not outrank the strongest security research simply because it is newer.

---

# 12. Skills-section integration

## SWE

Add:

**Rust**

Potentially add:
- Rayon
- Criterion
- Parallel Computing
- Performance Profiling

Do not remove strong existing C++/Linux/concurrency skills simply to make room for every Rust dependency.

## Quant

Add:

**Rust**

Strengthen:
- Monte Carlo Simulation
- Probabilistic Modeling
- Multithreading & Concurrency
- Performance Engineering
- Benchmarking

If the basketball model is genuinely Markovian, explicitly include:

**Markov Models**

Do not add it merely because the project description contains the word "Markov."

## Cybersecurity

Add:

**Rust**

Potentially add:
- Memory Safety
- Systems Programming
- Linux Performance Analysis

Do not turn the security skills section into a generic systems section.

---

# 13. Exact implementation-to-resume mapping

Antigravity should maintain this mapping while developing the project:

| Implemented feature | Resume evidence |
|---|---|
| Native Rust engine | Rust / systems programming |
| Possession-level Markov model | Markov modeling / state-transition simulation |
| Monte Carlo tournaments | Monte Carlo simulation |
| Deterministic keyed PRNG | reproducibility / deterministic computation |
| Cross-thread identical SHA-256 | deterministic parallel execution |
| Rayon | parallel execution / work stealing |
| Local accumulators | contention reduction |
| Deterministic reduction | reproducible aggregation |
| SoA | data-oriented design / memory layout |
| Criterion | statistical benchmarking |
| Linux perf | performance profiling |
| IPC measurement | CPU efficiency analysis |
| cache miss measurement | memory/cache analysis |
| SIMD | vectorized execution |
| Amdahl analysis | parallel scaling analysis |
| Tauri/React telemetry | instrumentation UI only |

The presence of a feature in this table does not mean it can automatically appear on the resume. It must actually be implemented and tested.

---

# 14. README should support the resume

The GitHub README should make the resume claims easy to verify.

Recommended top-level README structure:

```text
# Deterministic Parallel Basketball Simulation Engine

## What it is
## Architecture
## Basketball simulation model
## Determinism guarantee
## Parallel execution
## Benchmark methodology
## Scaling results
## Profiling results
## Correctness tests
## Reproducibility
## Visualization / telemetry
## Build and run
## Benchmark commands
## Results
```

The README should include actual benchmark tables and commands.

A recruiter/interviewer should be able to move from:

> "2.4× speedup"

on the resume to the repository and find exactly how that number was obtained.

---

# 15. Interview-readiness requirements

Before putting the project on any resume, Antigravity should make sure the user can explain:

### Basketball model
- Why possession-level simulation?
- Why Markov?
- What constitutes the state?
- What are the transitions?
- How are probabilities determined?
- Why not simply sample final scores from a Gaussian?

### Monte Carlo
- What is being estimated?
- Why does independent simulation parallelize well?
- What is variance?
- How does increasing simulation count affect confidence?

### Concurrency
- Why parallelize games rather than possessions?
- What data is shared?
- What is thread-local?
- Where is synchronization required?
- How are results reduced?

### Determinism
- Why can ordinary RNG streams break reproducibility?
- How does the keyed PRNG work?
- Why should thread count not affect results?
- Why is a hash useful?

### Performance
- Where is the bottleneck?
- What does IPC tell you?
- What do cache misses tell you?
- What is Amdahl's Law predicting?
- Why does scaling flatten?
- Is the workload CPU-bound or memory-bound?

### Rust
- ownership/borrowing
- `Send` / `Sync`
- thread safety
- slices and memory layout
- allocation behavior
- Rayon internals at a high level

If the user cannot answer these, the project is not yet "resume ready" even if the code works.

---

# 16. Critical rule for Antigravity

Do not optimize the project for impressive-sounding resume bullets.

Optimize it for:

**correctness → reproducibility → measurement → explanation → optimization**

The resume is the final compressed representation of that work.

The strongest version of this project is not:

> "Built a basketball simulator that runs 10M games/sec."

The strongest version is:

> "Built a deterministic parallel Monte Carlo engine, established a reproducible baseline, measured scaling and hardware behavior, identified bottlenecks, optimized the hot path, and can explain every number reported."

That distinction matters for the companies the user is targeting.

---

# 17. Final resume gate

Antigravity should only update a resume after all applicable checks pass:

- [ ] Project compiles from a clean checkout.
- [ ] Tests pass.
- [ ] Determinism test passes across multiple thread counts.
- [ ] Benchmark is reproducible.
- [ ] Performance numbers are measured on a documented machine.
- [ ] Every optimization is backed by a before/after measurement.
- [ ] No unimplemented technology appears in the skills/header.
- [ ] No fabricated performance claim exists.
- [ ] Resume bullet can be traced to code/benchmark/README evidence.
- [ ] Project remains understandable in a 20–30 second recruiter scan.
- [ ] Project can survive a technical interview about its architecture.

---

# 18. Bottom line

The user's three resumes already have strong role-specific framing:

- SWE: concurrency, Linux, systems, memory, software architecture.
- Quant: probability/statistics, Monte Carlo, concurrency, latency/performance.
- Cybersecurity: cryptography, reverse engineering, RF/hardware, low-level security.

The basketball engine should **amplify those existing narratives rather than create a fourth unrelated identity**.

The project should therefore be the same underlying engineering artifact with three different explanations:

**SWE:** deterministic parallel systems + performance engineering

**Quant:** Monte Carlo + stochastic modeling + parallel computation + empirical scaling

**Cybersecurity:** low-level systems + correctness/reproducibility + secure/reliable execution

Do not maintain three different codebases or artificially change the project's technical claims between resumes. Change the emphasis, not the facts.
