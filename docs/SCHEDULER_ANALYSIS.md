# Phase 19: Custom Work-Stealing Scheduler Analysis

## Overview
To deeply understand task distribution and the underlying mechanics of thread pools, we implemented a custom work-stealing scheduler from scratch using the `crossbeam-deque` crate (which implements the Chase-Lev lock-free deque). 

This scheduler was integrated into the simulation engine and can be toggled via the CLI (`--scheduler custom`).

## Architectural Comparison

### Rayon (`--scheduler rayon`)
- Uses a highly optimized, adaptive work-stealing pool.
- `into_par_iter().fold()` automatically splits the 100,000 simulations into optimally sized chunks based on the number of idle threads.
- Implements sophisticated thread parking (sleeping) and exponential backoff when queues are empty to avoid burning CPU cycles spinning on locks.

### Custom Scheduler (`--scheduler custom`)
- **Static Chunking**: We explicitly chunked the 100,000 simulations into fixed "morsels" of 500 simulations each.
- **Chase-Lev Deque**: Each thread owns a `crossbeam_deque::Worker` (FIFO/LIFO queue). 
- **Naive Stealing**: If a thread finishes its local queue, it iterates through an array of `Stealer` handles attached to sibling threads and attempts to `steal_batch_and_pop`.
- **Spinning**: Threads vigorously poll sibling queues without exponential backoff or parking, which creates massive cache-coherence traffic on the atomic variables controlling the deques.

## Empirical Benchmark Results

We ran 100,000 simulations on 16 threads (measuring the branch-predictor optimized codebase from Phase 17):

| Scheduler | Runtime | Game Throughput | Delta vs Rayon |
| :--- | :--- | :--- | :--- |
| **Rayon (Baseline)** | 4.39s | 2,020,755 games/sec | - |
| **Custom Work-Stealing (Naive)** | 6.51s | 1,364,643 games/sec | -32.4% |
| **Custom Work-Stealing (Backoff + Micro-Chunks)** | 9.00s | 987,423 games/sec | -51.1% |

## Conclusion

Our initial custom scheduler successfully distributed the workload but was **32.4% slower** than Rayon. 

We then attempted to "supercharge" the custom scheduler by introducing exponential backoff (`crossbeam_utils::Backoff`) to eliminate cache-ping-ponging during idle states, and we decreased the static chunk size from 500 to 50 to theoretically improve load balancing.

**The result was a disaster: throughput plummeted to 987,423 games/sec (51.1% slower than Rayon).**

### Why did our optimizations fail?
1. **The Overhead Razor's Edge**: By dropping the chunk size to 50, we forced the threads to interact with the atomic `push` and `pop` operations of the Chase-Lev deque 10x more frequently. The queue overhead completely dominated the extremely fast math of the simulation.
2. **Backoff Penalty**: Because our chunks were so small, threads finished them instantly. They immediately entered the steal loop, missed, and engaged the exponential backoff. This forced threads to `yield` to the OS constantly, destroying their momentum and keeping the CPU cores idle when they should have been working.

This experiment definitively proves the incredible engineering depth behind the Rayon crate. Writing a thread pool is easy; writing a *fast* thread pool requires years of tuning adaptive chunking algorithms and heuristic backoff strategies. We will retain the custom scheduler in the codebase for educational purposes, but the production engine will permanently default to Rayon.
