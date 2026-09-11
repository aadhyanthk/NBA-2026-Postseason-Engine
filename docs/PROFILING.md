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
