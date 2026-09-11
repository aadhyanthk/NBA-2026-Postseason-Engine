# Tree Reduction and Lock-Free State Accumulation

## The Problem with Global Mutexes
When processing millions of simulations in parallel, a naive approach is to spawn $N$ threads, have each thread run a simulation, and then lock a global `Mutex<[u32; 30]>` to update the scoreboard. 

In a system processing hundreds of thousands of games per second, the time spent fighting for this single lock (lock contention) drastically outweighs the time spent simulating the games. Furthermore, multiple cores constantly modifying the same memory address causes severe **cache-line bouncing**, forcing CPU caches to constantly flush and invalidate across the silicon die.

## The Solution: Local Accumulators & Tree Reduction
To avoid shared mutable state entirely, we leverage **Data-Parallel Tree Reduction** (handled natively by the `rayon` crate via `fold` and `reduce`).

1. **`fold` (Thread-Local Accumulators):**
   Instead of modifying a global array, each worker thread is handed its own zeroed-out array. When a worker simulates a batch of games, it strictly updates its *own* local array. Because the thread owns this memory exclusively, it requires no locks and stays hot in the L1 cache.

2. **`reduce` (Tree-Reduction):**
   When the workers finish their workloads, their local arrays must be combined into the final global state. Rather than sequentially adding them one by one, the arrays are merged hierarchically in a tree structure. 
   - `Array A` merges with `Array B` to form `Array AB`.
   - `Array C` merges with `Array D` to form `Array CD`.
   - Finally, `Array AB` and `Array CD` are merged to form the final array.

This entire pipeline allows the engine to achieve near-linear parallel scaling because worker threads operate 100% independently until the very final millisecond of execution.

## Floating-Point vs Integer Consistency
We strictly use integer counters (`u32` and `u64`) for all scoreboard tallies. If we used floating points, the reduction tree order (which is non-deterministic based on thread finish times) would cause microscopic rounding errors due to floating-point non-associativity ($ (a + b) + c \neq a + (b + c) $ in IEEE 754). Integers guarantee perfect mathematical associativity, preserving our strict bit-for-bit determinism constraint.
