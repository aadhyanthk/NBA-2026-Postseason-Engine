# Multi-Core Scaling Benchmarks (Phase 10 & 11)

## Setup
- **Workload**: 100,000 NBA Postseasons (approx ~8,500,000 individual basketball games)
- **Compiler**: `rustc` with `lto = "fat"`, `codegen-units = 1`, `opt-level = 3`
- **CPU Bound**: Pure simulation loop (zero dynamic heap allocations)

## Results

| Threads | Runtime | Speedup | Parallel Efficiency | Postseasons/sec |
|---------|---------|---------|---------------------|-----------------|
| 1       | 47.28s  | 1.00x   | 100%                | 2,115           |
| 2       | 31.53s  | 1.50x   | 75.0%               | 3,171           |
| 4       | 16.62s  | 2.84x   | 71.0%               | 6,014           |
| 8       | 12.81s  | 3.69x   | 46.1%               | 7,805           |
| 16      | 8.47s   | 5.58x   | 34.8%               | 11,803          |

## Amdahl's Law Analysis

By scaling up the thread count, we can observe the law of diminishing returns predicted by **Amdahl's Law**:
$Speedup(N) = \frac{1}{(1 - P) + \frac{P}{N}}$

Where `P` is the parallelizable portion of the workload.

Even though our engine employs a completely lock-free Tree Reduction model to avoid shared mutable state (`Phase 8 & 9`), it does not scale perfectly linearly at high thread counts (e.g., 16 threads is not exactly 16.0x faster). This is due to:

1. **Memory Bandwidth (The Von Neumann Bottleneck):** Even with local cache-lines, as 16 cores request instructions and data, the CPU's internal ring-bus and memory controller limits total bandwidth.
2. **OS Scheduling Overhead:** The Rayon work-stealing scheduler takes a microscopic fraction of a millisecond to distribute workloads.
3. **Serial Reduction Phase:** Combining the 16 independent array scoreboards at the very end is inherently serial work.
4. **Thermal Throttling:** 100% load on 16 threads causes the CPU frequency to boost and eventually throttle down slightly, meaning high-core loads have lower single-thread frequencies than single-thread loads.
