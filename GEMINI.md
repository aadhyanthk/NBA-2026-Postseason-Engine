# GEMINI.md — NBA 2026 Postseason Systems Engine Operating Manual

This file governs all interactions and implementations within this workspace. Read and follow these directives before answering any user request or generating code.

---

## 1. Core Mission & Persona

- **Project Identity**: `NBA-2026-Postseason-Engine` — A native Rust, high-throughput, deterministic, multi-threaded Monte Carlo simulation engine modeling the 2026 NBA Postseason (Play-In Tournament & Best-of-7 Playoff series).
- **Target Standard**: Unimpeachable systems engineering quality that withstands a brutal 30-minute technical grilling by an HFT or low-level systems engineer.
- **Output Aesthetics**: Whatever the user sees (CLI output, HTML reports, documentation) MUST NOT look "vibe-coded". It must look as close to industry-grade professional software as possible. Use clean terminal tables (e.g. `comfy-table` or well-aligned formatting), clear reporting, and rigorous scientific precision.
- **Tone**: Rigorous, systems-first, precise, honest, anti-hype.

---

## 2. Reverse Prompting & Pre-Implementation Protocol (MANDATORY)

Before implementing or modifying code:
1. **Never Assume Ambiguous Requirements**: If a design choice, mathematical model, memory layout, or concurrency approach has multiple valid paths, **reverse-prompt the user** with a concise analysis of the tradeoffs before writing code.
2. **Phase Clarification & Alignment**: Break down what will be built, why it's designed that way, how it will be tested, and what baseline it establishes.
3. **Structured Review**: Explicitly state:
   - *Technical Motivation*
   - *Memory/Cache Implications*
   - *Determinism Invariant Handling*
   - *Verification Strategy*

---

## 3. Systems Engineering Principles & Defense Checklist

Every technical decision must answer: **“Why did you design it this way?”** without hiding behind high-level third-party library abstractions.

### A. Strict Determinism Under Arbitrary Parallelism
- Running `./nba-sim --seed 42 --threads 1` and `./nba-sim --seed 42 --threads 32` **must produce 100% bit-for-bit identical results**.
- **No Shared Global PRNG**: Use counter-based or splittable PRNGs (e.g. Philox4x32 or PCG64) indexed hierarchically: `Hash(MasterSeed, SimulationIndex, SeriesIndex, GameIndex, PossessionIndex)`.
- Thread execution and work-stealing order must never dictate the random stream.

### B. Memory Architecture & Cache Awareness
- **Hot-Path Zero-Allocation**: No `malloc`/`free`, `Box`, `Vec::push`, or dynamic resizing inside the per-possession / per-game simulation loop. Pre-allocate thread-local simulation arenas.
- **Data-Oriented Design (SoA vs AoS)**: Structure team and game states contiguously to optimize L1/L2 data cache hit rates and hardware prefetching.
- **False-Sharing Elimination**: Thread-local aggregators must be cache-line aligned (`#[repr(align(64))]`).
- **Reduction Strategy**: Use lock-free, cache-padded thread accumulators followed by a deterministic parallel tree reduction.

### C. Optimization Discipline (No Premature / Fake Optimizations)
Do not implement SIMD, custom allocators, or lock-free queues without:
1. A concrete performance bottleneck identified via profiling (`perf`, flamegraphs, or criterion).
2. A documented baseline.
3. A pre-optimization measurement.
4. A post-optimization measurement.
5. A written systems explanation of the speedup/slowdown.

---

## 4. Domain Model: 2026 NBA Postseason

- **Scope**:
  - 30 NBA Teams (Eastern & Western Conferences).
  - **Play-In Tournament**: Seeds 7 vs 8 (winner gets #7), Seeds 9 vs 10 (loser eliminated), Elimination match (winner gets #8).
  - **Playoff Bracket**: 16 teams (8 per conference), 4 rounds of Best-of-7 series (2-2-1-1-1 home court format).
  - **Match/Game Simulation Model**: Possession-based or Four-Factors Markov model (Pace, Offensive Rating, Defensive Rating, Home Court Advantage, Turnover/Rebound probabilities, 3P/2P/FT shooting distributions).
- **Scale Goals**:
  - 100,000 to 10,000,000 full postseason Monte Carlo runs simulated in seconds.

---

## 5. Code Quality & Cleanliness Standards

- **Language**: Idiomatic, high-performance Rust (`--release` optimized, strictly typed, zero unhandled errors/panics).
- **Modularity**: Small, single-responsibility modules under `src/` or `crates/`.
- **Testing**:
  - Unit tests for game rules, play-in progression, and series win criteria.
  - Property tests (`proptest`) for tournament invariants (e.g. exactly one champion per simulation, valid seed progression).
  - Dedicated CLI determinism verification subcommand (`verify-determinism`) utilizing SHA-256 byte comparisons across thread counts `[1, 2, 4, 8, 16, 32]`.

---

## 6. Anti-Vibe-Coding & Visual Aesthetics Standards (MANDATORY)

Whatever the user sees (CLI output, HTML reports, documentation, logs) MUST NOT look "vibe-coded". It must reflect an industry-grade, professional systems engineering pedigree. 

- **CLI / Terminal Output**: 
  - Never print unaligned, sloppy debug dumps to the user.
  - Always use properly structured, border-aligned tables (e.g., `comfy-table`).
  - Use clear tabular headers: `[Runtime]`, `[Throughput (games/sec)]`, `[Parallel Efficiency]`.
  - Use appropriate decimal precision (e.g. `23,041 ps/s`, not `23041.49219491 ps/s`).
- **Reports & Artifacts**:
  - HTML or Markdown reports must feel like technical papers or institutional database readouts. 
  - Do not use generic, unstyled web outputs. Employ clean CSS with strong typography (e.g., Inter, JetBrains Mono), readable tables, and rigorous data presentation.
- **Tone & Messaging**:
  - No emojis in serious log lines.
  - No conversational or casual phrasing in system alerts (e.g., say `[WARNING] Cache-line alignment failed`, not `Whoops, alignment broke!`).

---

## 7. Git Commit & Push Protocol (MANDATORY)

- **Continuous Version Control**: Commits and pushes to GitHub must be made whenever **ANY** change is completed.
- **Commit Format**: Follow standard Conventional Commits format:
  - `feat(<scope>): <description>` — New feature / simulation capability.
  - `perf(<scope>): <description>` — Measured performance / data layout optimization.
  - `test(<scope>): <description>` — Correctness, determinism, or property test additions.
  - `docs(<scope>): <description>` — Documentation, architecture, benchmarks, model design.
  - `refactor(<scope>): <description>` — Code refactoring without behavioral changes.
  - `chore(<scope>): <description>` — Build config, tooling, CI.

---

## 7. Project Plan Progress Tracking (MANDATORY)

- As phases and milestones in `NBA_Systems_Project_Plan.md` (and `FIFA_Systems_Project_Plan.md`) are executed and completed, they must immediately be updated with a checkmark `[x]` (e.g. `## Phase 0 — [x] Repository Audit`).
- Never leave a finished phase unmarked.
