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
| **Custom Work-Stealing** | 6.51s | 1,364,643 games/sec | -32.4% |

## Conclusion

Our custom scheduler successfully distributed the workload, achieved 100% correct deterministic output, and ran the full 100,000 postseasons in ~6.5 seconds.

However, it was **32.4% slower** than Rayon. 

### Why did the custom scheduler lose?
1. **Contention on Stealing**: Because our threads do not use exponential backoff, when they run out of work, they aggressively poll the atomic pointers of sibling deques. This causes L1 cache invalidations across cores, starving the threads that are actually trying to execute simulation math.
2. **Static Chunk Sizing**: A fixed chunk size of 500 means the OS is spending more time context switching and running queue synchronization code than it needs to. Rayon dynamically adjusts its chunk sizes (often starting large and getting smaller as the queue drains) to perfectly balance overhead vs load balancing.

This experiment proves the incredible engineering depth behind the Rayon crate. We will retain the custom scheduler in the codebase for educational purposes, but the production engine will continue defaulting to Rayon.
