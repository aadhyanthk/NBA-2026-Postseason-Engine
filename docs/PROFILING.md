# Profiling & Arithmetic Optimization (Phase 15)

## Hypothesis
During `Phase 12` we discovered that the CPU was simulating 1.6 Million games per second, but inside the innermost loop (`simulate_possession`), we were dynamically calculating `p3`, `p2`, and `p1` shot distributions. Because possession math involves floating-point divisions (`expected_ppp / (1.0 - tov_pct)`), we hypothesized that the CPU's arithmetic logic units (ALUs) were bottlenecking on redundant divisions.

Since a basketball game has ~200 total possessions, and the four factors (`expected_ppp`, `tov_pct`, `three_point_rate`) are completely static for a given matchup, the CPU was performing tens of thousands of redundant floating-point divisions per simulated game.

## Measurement
- **Baseline (Phase 12 SoA layout):** 1,602,472 games/sec

## Change
We refactored `src/sim/game.rs` to hoist the probability distribution math out of the `for _ in 0..total_possessions` loop. We created a helper `calc_shot_probs` that is called exactly twice per game (once for Home, once for Away). The pre-calculated probabilities are then passed down into the `simulate_possession` loop, totally eliminating the division operations from the hot path.

## Result
- **New Throughput:** 1,686,061 games/sec
- **Improvement:** +83,589 games/sec

## Explanation
Floating-point division is one of the slowest operations on modern CPUs (often taking 10-15 cycles compared to 1-3 cycles for addition or multiplication). By doing the math upfront, the inner loop now consists entirely of pseudo-random generation, float comparisons (`draw < p3`), and simple integer additions. This allows the ALUs to pipeline instructions far more aggressively without stalling on the divider block.

---

# Branch Prediction & Arithmetic Selection (Phase 17)

## Hypothesis
Inside `simulate_possession`, the engine historically relied on an `if / else if / else if` cascade to determine whether a shot was a 3-pointer, 2-pointer, or 1-pointer. Because the `draw` variable is purely stochastic (`gen_range(0.0, 1.0)`), the hardware branch predictor is forced to guess the outcome of a weighted coin flip. When the predictor guesses wrong, the CPU must flush its deep instruction pipeline, incurring a massive ~15-20 cycle penalty.

## Measurement
- **Baseline (Phase 16 - Native Auto-Vec):** 1,771,595 games/sec (5.01s per 100k postseasons)

## Change
We refactored `simulate_possession` to use completely branchless arithmetic selection. Instead of branching, we use boolean-to-integer casting (which compiles to zero-overhead conditional moves or bitmasks):
```rust
let pts = ((draw < p3) as u16 * 3)
        + ((draw >= p3 && draw < p2_threshold) as u16 * 2)
        + ((draw >= p2_threshold && draw < p1_threshold) as u16 * 1);
```

## Result
- **New Throughput:** 2,020,755 games/sec (4.39s per 100k postseasons)
- **Improvement:** +249,160 games/sec (+14.0%)

## Explanation
By completely eliminating the branch misprediction penalty in the core hot loop, the CPU pipeline is never flushed. The CPU evaluates all three conditions mathematically, but because multiplication by `0` clears the unused branches and scalar arithmetic is incredibly fast (1 cycle), the raw throughput vastly exceeds the unpredictable branching model. We broke the 2 million games-per-second barrier!
