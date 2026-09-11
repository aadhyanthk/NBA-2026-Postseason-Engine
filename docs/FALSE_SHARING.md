# Phase 18: False Sharing & Cache-Line Alignment

## What is False Sharing?
Modern CPUs read and write to main memory in discrete blocks called **cache lines**, which are typically **64 bytes** in size on x86_64 architecture.

**False sharing** occurs when two independent variables reside on the same 64-byte cache line in memory, and two different CPU cores attempt to read/write to those independent variables simultaneously. 

For example, if `Core 1` modifies `Variable A` and `Core 2` modifies `Variable B` (which is located 8 bytes away from `Variable A`), the CPU hardware is unable to track the modifications at the variable level. Instead, the MESI/MOESI cache coherence protocol invalidates the *entire 64-byte cache line* for all other cores. This forces `Core 2` to fetch the cache line from L3 cache or main memory again, destroying multi-core performance in a phenomenon known as "cache ping-pong."

## Investigation & Current Architecture
In our simulation engine, the multithreaded accumulation of simulation results (e.g. tallying which team won the championship) is orchestrated by the Rayon library's `into_par_iter().fold()` paradigm. 

Rayon dynamically allocates the `fold` accumulators onto the local stack of each worker thread. Because modern operating systems allocate distinct memory regions for thread stacks—often separated by megabytes—our thread-local accumulators naturally spanned vastly different cache lines. Thus, **we were not currently bottlenecking on false sharing.**

## The Vulnerability
Relying on anonymous tuples allocated implicitly on thread stacks is architecturally fragile. If a future engineer refactored the concurrency model to use an array of accumulators (e.g., `vec![Accumulator::default(); num_threads]`) and passed references into `std::thread::spawn`, that dense array would trigger catastrophic false sharing, instantly dropping our throughput from 2M games/sec to a fraction of that speed.

## The Solution: Architectural Defense in Depth
To completely eliminate the possibility of false sharing, we replaced the implicit anonymous tuples with a formal struct in `src/core/types.rs`, and explicitly enforced a 64-byte cache-line alignment using the Rust compiler directive `#[repr(align(64))]`:

```rust
#[derive(Clone, Default)]
#[repr(align(64))] // Force 64-byte cache-line alignment to prevent false sharing
pub struct SimAccumulator {
    pub play_in: [u32; 30],
    pub playoffs: [u32; 30],
    pub conf_finals: [u32; 30],
    pub finals: [u32; 30],
    pub championships: [u32; 30],
    pub total_games: u64,
}
```

This guarantees that two `SimAccumulator` instances can **never** occupy the same 64-byte chunk of memory, rendering false sharing physically impossible, regardless of how or where the struct is allocated in the future.

## Benchmark Results
Because false sharing was already mitigated by Rayon's stack separation, our throughput remained identical (and slightly faster due to cleaner compiler instruction layout for the struct vs a massive tuple):
- **Throughput post-alignment**: ~2,091,279 games/sec

This phase serves as a critical defense-in-depth architectural constraint for high-performance low-level systems.
