# Basketball Simulation Engine — UI Design & Implementation Plan

## 0. Purpose

This document specifies the desktop UI surrounding the native Rust basketball simulation engine.

The UI is **not the product itself**. The simulation engine is the product. The UI exists to expose real engine controls, visualize real simulation output, demonstrate deterministic execution, expose performance telemetry, and make benchmark results understandable.

The visual target is **instrumentation software**, not a consumer sports dashboard.

Think:
- HPC monitoring tools
- trading terminals
- profiler interfaces
- infrastructure dashboards
- scientific instrumentation

Do **not** make it look like:
- a sports-betting website
- a fantasy-sports dashboard
- an AI SaaS landing page
- a generic React admin template
- a component-library showcase

### Central design rule

> Every visual element must correspond to real state or real data produced by the simulation engine.

No fake counters, fake CPU utilization, fake progress, decorative graphs, or client-side simulation logic.

---

# 1. Product Principles

## 1.1 Engine first

Rust remains authoritative.

React/Tauri must not contain:
- simulation algorithms
- fake simulation state
- client-side probability calculations
- fake timers used to simulate engine activity
- fake CPU utilization
- fake throughput
- mock benchmark values

Architecture:

```text
                 React / TypeScript
              Controls + Visualization
                        │
                     Tauri IPC
                        │
                        ▼
                 Native Rust Engine
                        │
        ┌───────────────┼────────────────┐
        │               │                │
   Basketball       Monte Carlo      Telemetry
   simulation       execution        + results
        │               │                │
        └───────────────┼────────────────┘
                        │
                        ▼
                  React rendering
```

---

# 2. Visual Identity

## 2.1 Overall aesthetic

Use a dark, dense, technical interface.

The application should immediately communicate:

> "This is a native compute engine being instrumented."

It should not communicate:

> "This is a pretty sports analytics app."

Recommended:
- dense information layout
- sharp or minimally rounded containers
- thin borders
- restrained contrast
- monospace numeric output
- compact labels
- left-aligned information
- visible units
- explicit states
- minimal animation
- no decorative gradients
- no giant hero numbers
- no excessive whitespace

Avoid the common AI-generated UI pattern of large rounded cards, soft shadows, oversized KPIs, and decorative gradients.

Prefer an instrumentation pattern:

```text
THROUGHPUT
────────────────────────────────────────
184,293 games/s

COMPLETED   704,321 / 1,000,000
ELAPSED     00:03.821
WORKERS     32

01 ████████████████████████ 96%
02 ██████████████████████   91%
03 ███████████████████████  94%
...
32 ████████████████████████ 97%
```

---

# 3. Color System

Use a deliberately small palette.

## 3.1 Base colors

```text
--bg-primary:     #0A0B0D
--bg-secondary:   #111317
--bg-panel:       #15181D
--border:         #2A2D33
--border-subtle:  #1C1E22

--text-primary:   #E6E8EB
--text-secondary: #969BA3
--text-muted:     #626872
```

## 3.2 Single accent

Use **one** primary accent.

Recommended default:

```text
--accent: #3B82F6
```

Use it for active controls, selected states, and important non-semantic data visualization.

Do not introduce multiple competing accent colors.

## 3.3 Semantic colors

Green means:
- verified
- correct
- pass

Red means:
- mismatch
- failure
- invalid

Do not use these colors decoratively.

---

# 4. Typography

Use two typefaces.

## 4.1 Data / numeric font

Preferred:
- JetBrains Mono
- Berkeley Mono if available

Use for:
- simulation counts
- seeds
- thread counts
- throughput
- elapsed time
- CPU/worker utilization
- SHA-256 digests
- benchmark values
- percentages
- logs
- IDs

This is functional: fixed-width numerical output is easier to scan and compare.

## 4.2 Interface font

Preferred:
- Inter

Use for:
- section titles
- labels
- explanatory text
- buttons
- navigation
- descriptions

Do not use oversized marketing typography.

---

# 5. Layout

Use a multi-panel instrumentation layout.

Suggested structure:

```text
┌─────────────────────────────────────────────────────────────────────┐
│ BASKETBALL SIMULATION ENGINE          ENGINE: READY     v0.x.x      │
├─────────────────────────────────────────────────────────────────────┤
│ CONTROL BAR                                                         │
│ SEED [42]   SIMULATIONS [1,000,000]   THREADS [32]  [VERIFY] [RUN]│
├──────────────────────────────┬──────────────────────────────────────┤
│ LIVE THROUGHPUT              │ DETERMINISM VERIFICATION             │
│ games/sec                    │ 1T   SHA-256 ...       ✓            │
│ core/worker utilization      │ 2T   SHA-256 ...       ✓            │
│ elapsed                      │ 4T   SHA-256 ...       ✓            │
│ per-worker bars              │ 8T   SHA-256 ...       ✓            │
│                              │ ...                                  │
├──────────────────────────────┼──────────────────────────────────────┤
│ BRACKET / ODDS               │ BENCHMARK / PERFORMANCE              │
│ tournament results           │ speedup                              │
│ win probabilities            │ IPC                                  │
│ series lengths               │ cache misses                         │
│                              │ benchmark measurements               │
└──────────────────────────────┴──────────────────────────────────────┘
│ LIVE ENGINE LOG                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

Exact proportions may change based on the existing repository, but the application should remain information-dense.

---

# 6. Panel 1 — Control Bar

The control bar is the primary interaction surface.

It must expose parameters that actually affect an engine run.

## Required controls

### Seed

```text
SEED
[ 42 ]
```

Maps directly to the Rust simulation seed.

### Simulation count

```text
SIMULATIONS
[ 1,000,000 ]
```

Maps directly to the Monte Carlo workload.

### Thread count

```text
THREADS
[ 32 ]
```

Controls Rust execution configuration.

Possible options:

```text
1 / 2 / 4 / 8 / 16 / 32
```

Also allow automatic/native maximum if supported.

### Determinism verification

```text
[✓] VERIFY DETERMINISM
```

When enabled, run the same workload at multiple thread counts and compare resulting digests.

### Run button

```text
[ RUN SIMULATION ]
```

Starts a real Rust engine execution.

## Run states

Clearly distinguish:

```text
READY
RUNNING
VERIFYING
COMPLETE
ERROR
```

Do not allow conflicting runs unless the backend explicitly supports them.

---

# 7. Panel 2 — Live Throughput Telemetry

Show what the native engine is actually doing.

Required data:
- games completed
- total simulations
- games/sec
- elapsed time
- estimated remaining work where meaningful
- thread count
- per-worker/per-core utilization if the backend can provide it

Example:

```text
LIVE THROUGHPUT
────────────────────────────────────

THROUGHPUT
184,293 games/s

COMPLETED
704,321 / 1,000,000

ELAPSED
00:03.821

WORKERS
32

WORKER UTILIZATION

01 ████████████████████████ 96%
02 ██████████████████████   91%
03 ███████████████████████  94%
...
32 ████████████████████████ 97%
```

## Sampling

Telemetry should be sampled around 10 Hz.

Do not emit an IPC event for every simulated game.

Preferred flow:

```text
simulation workers
       ↓
local counters
       ↓
telemetry aggregation
       ↓
~10 Hz snapshots
       ↓
Tauri event
       ↓
React
```

This keeps the UI from becoming the bottleneck.

---

# 8. Telemetry Event Contract

The Rust side should expose a serializable progress structure.

Conceptual schema:

```rust
struct SimulationProgress {
    run_id: String,
    games_completed: u64,
    games_total: u64,
    elapsed_ns: u64,
    games_per_second: f64,
    worker_count: usize,
    worker_utilization: Vec<f32>,
}
```

Exact implementation may differ.

Requirements:
1. Values originate from the engine.
2. Units are explicit.
3. Counters are never invented by React.
4. Telemetry collection has negligible impact on benchmark-critical execution.
5. Benchmark mode can disable UI telemetry when necessary.

---

# 9. Panel 3 — Determinism Verification Console

## Highest-priority UI component

The project claims deterministic parallel execution. The UI should make that claim directly inspectable.

Verification workflow:

```text
seed = 42
simulations = 1,000,000

run with 1 thread
run with 2 threads
run with 4 threads
run with 8 threads
run with 16 threads
run with 32 threads

compare output digests
```

Expected display:

```text
DETERMINISM VERIFICATION
────────────────────────────────────────────────────────

CONFIG
SEED          42
SIMULATIONS   1,000,000

THREADS     SHA-256 DIGEST                         STATUS

1           91d4c...a82                           ✓ MATCH
2           91d4c...a82                           ✓ MATCH
4           91d4c...a82                           ✓ MATCH
8           91d4c...a82                           ✓ MATCH
16          91d4c...a82                           ✓ MATCH
32          91d4c...a82                           ✓ MATCH

RESULT
✓ ALL CONFIGURATIONS PRODUCED IDENTICAL OUTPUT
```

Mismatch:

```text
32          7a13f...991                           ✗ MISMATCH

RESULT
✗ DETERMINISM FAILURE
EXPECTED   91d4c...a82
ACTUAL     7a13f...991
```

The mismatch state must be visually obvious.

### Important distinction

SHA-256 is a verification mechanism, not the source of determinism.

Determinism comes from the engine architecture:
- deterministic seed handling
- deterministic RNG
- simulation identity
- stable result reduction
- avoiding nondeterministic floating-point behavior where relevant

The UI exposes the evidence.

---

# 10. Determinism API

Expose a backend operation conceptually equivalent to:

```text
verify_determinism(
    simulations,
    seed,
    thread_counts
)
```

Return:

```text
{
    seed,
    simulations,
    runs: [
        {
            threads,
            digest,
            elapsed_ns
        }
    ],
    all_match
}
```

The frontend renders this result.

Do not calculate digests in TypeScript.

---

# 11. Panel 4 — Basketball Bracket / Odds

This is the domain-facing portion of the application.

It demonstrates that the simulation engine is actually modeling basketball.

Display real Monte Carlo output such as:
- tournament bracket
- team advancement probabilities
- matchup win probabilities
- expected series length
- championship probability
- final score distributions
- selected game details

Example:

```text
TOURNAMENT
────────────────────────────────────────────────────────

ROUND 1             SEMIFINAL           FINAL

A ─────┐
       ├──── A ─────┐
B ─────┘            │
                    ├──── A 61.2%
C ─────┐            │
       ├──── C ─────┘
D ─────┘
```

Probability bars:

```text
TEAM A   ████████████████████████ 61.2%
TEAM B   ███████████████          38.8%
```

Series-length distribution:

```text
SERIES LENGTH

4 GAMES   ██████████████       31.4%
5 GAMES   █████████████████    38.2%
6 GAMES   ███████████          22.8%
7 GAMES   ████                  7.6%
```

If the engine does not support a feature, do not fabricate the visualization.

---

# 12. Basketball Data Model Boundary

The frontend must not implement basketball rules.

Rust owns:
- possession simulation
- event transitions
- scoring
- fouls
- rebounds
- turnovers
- game clock
- team strengths
- tournament progression
- random sampling
- aggregation

Frontend receives results such as:
- team
- win probability
- score distribution
- series distribution
- tournament advancement probability

The UI only renders them.

---

# 13. Panel 5 — Benchmark / Performance

Expose measured systems performance.

Potential metrics:
- single-thread baseline
- multithread throughput
- speedup
- parallel efficiency
- IPC
- cache misses
- branch mispredictions
- allocation count
- benchmark confidence/error where applicable

Example structure:

```text
PERFORMANCE
────────────────────────────────────────────

THREADS      GAMES/S       SPEEDUP       EFFICIENCY

1             [actual]       1.00x          100%
2             [actual]       [actual]       [actual]
4             [actual]       [actual]       [actual]
8             [actual]       [actual]       [actual]
16            [actual]       [actual]       [actual]
32            [actual]       [actual]       [actual]
```

**Never put fabricated benchmark numbers into the UI.**

---

# 14. Amdahl Speedup Visualization

Display measured speedup against thread count.

Axes:
- X = thread count
- Y = speedup

Optionally include:
1. measured engine speedup;
2. ideal linear scaling;
3. theoretical Amdahl prediction if the serial fraction is actually measured/estimated.

Do not present theoretical speedup as measured performance.

Use Recharts, D3, or another lightweight chart library if useful.

Keep the chart visually restrained.

---

# 15. Performance Data Ingestion

Initial implementation can use exported Criterion results.

## Phase 1

```text
Criterion output
      ↓
JSON parser
      ↓
React benchmark panel
```

## Phase 2

Generate a normalized benchmark artifact such as:

```json
{
  "benchmark": "simulate_1m_games",
  "threads": 16,
  "games": 1000000,
  "elapsed_ns": 612345678,
  "games_per_second": 1633074
}
```

## Phase 3

If useful, expose benchmark execution directly through Tauri.

Do not make benchmark execution part of the ordinary interactive path unless carefully isolated.

---

# 16. Charts

Use charts only where they communicate information better than text.

Good candidates:
- thread scaling
- speedup
- efficiency
- score distribution
- series-length distribution
- win probability

Avoid:
- pie charts for trivial information
- decorative gauges
- 3D charts
- animated charts
- rainbow series
- decorative chart backgrounds

---

# 17. Worker Utilization Visualization

Do not use a generic chart-library bar chart for dozens of workers.

Use a compact custom visualization:

```text
WORKER UTILIZATION

01 ████████████████████████████ 96%
02 ██████████████████████████   91%
03 ███████████████████████████  94%
04 ████████████████████████████ 98%
...
32 ███████████████████████████  95%
```

For large worker counts, use a grid.

Do not claim these are physical CPU-core measurements if they are actually worker utilization.

The label must match what the backend measures.

---

# 18. Live Engine Log

A compact terminal-like log should appear at the bottom.

Example:

```text
ENGINE LOG
────────────────────────────────────────────────────────

15:32:04.221  RUN       seed=42 sims=1000000 threads=32
15:32:04.223  ENGINE    initialized 32 workers
15:32:04.224  RNG       deterministic mode enabled
15:32:05.109  PROGRESS  250000 / 1000000
15:32:05.874  PROGRESS  500000 / 1000000
15:32:06.601  PROGRESS  750000 / 1000000
15:32:07.412  COMPLETE  elapsed=3.188s throughput=313674 games/s
15:32:07.413  HASH      91d4c...a82
```

Requirements:
- real timestamps
- real engine events
- bounded log buffer
- newest entries visible
- monospace font
- no fake messages

---

# 19. Tauri / React Architecture

## Backend

Rust owns:

```text
simulation
configuration
RNG
parallel execution
aggregation
determinism
benchmark execution
telemetry
```

Tauri provides:

```text
IPC commands
IPC events
serialization
application lifecycle
```

## Frontend

React owns:

```text
rendering
controls
view state
display formatting
chart rendering
panel layout
user interaction
```

Zustand may own frontend state such as:

```text
current run
engine status
configuration
telemetry snapshot
determinism results
simulation results
benchmark results
log entries
```

Zustand must not become a second simulation engine.

---

# 20. Tauri Command Interface

Implement a clean command boundary.

Conceptual commands:

```rust
run_simulation(...)
get_simulation_result(...)
verify_determinism(...)
load_benchmarks(...)
```

Potential events:

```text
simulation://progress
simulation://log
simulation://complete
```

with serializable payloads.

The API should clearly separate:
- commands
- progress events
- completion
- errors

---

# 21. Streaming Progress

Do not block the UI waiting for a single huge IPC response when a run is long enough for live telemetry to matter.

Preferred flow:

```text
User clicks RUN
        ↓
Tauri command starts engine job
        ├──────────────→ progress events
        │                     ↓
        │                  React UI
        │
        └──────────────→ completion event
                              ↓
                         final result
```

The frontend remains responsive while the simulation runs.

Ensure the actual simulation workers are not accidentally serialized behind the UI runtime.

---

# 22. Frontend State Model

Conceptual Zustand state:

```typescript
type EngineStatus =
  | "idle"
  | "running"
  | "verifying"
  | "complete"
  | "error";

interface SimulationConfig {
  seed: number;
  simulations: number;
  threads: number;
  verifyDeterminism: boolean;
}

interface TelemetrySnapshot {
  completed: number;
  total: number;
  gamesPerSecond: number;
  elapsedNs: number;
  workerUtilization: number[];
}

interface DeterminismRun {
  threads: number;
  digest: string;
  elapsedNs: number;
  match: boolean;
}
```

Do not copy massive simulation state into React if unnecessary.

Prefer compact snapshots.

---

# 23. UI Update Frequency

Target approximately:

```text
~10 updates/sec
```

The backend should aggregate counters before emitting updates.

Do not let telemetry update frequency dominate rendering or distort engine performance.

---

# 24. Animation Rules

Animation is allowed only when it communicates real state.

Good:
- progress changing because real work completed
- utilization changing because telemetry changed
- digest changing from pending → verified
- status changing READY → RUNNING → COMPLETE

Bad:
- cards floating on hover
- animated gradient backgrounds
- decorative particles
- fake number-counting animations
- chart animations that obscure data
- spinning elements pretending to represent computation

Rule:

> No animation without a corresponding engine/UI state transition.

---

# 25. Component Structure

Suggested React structure:

```text
src/
├── app/
│   └── App.tsx
│
├── components/
│   ├── shell/
│   │   ├── Header.tsx
│   │   └── ControlBar.tsx
│   │
│   ├── telemetry/
│   │   ├── ThroughputPanel.tsx
│   │   ├── WorkerUtilization.tsx
│   │   └── EngineStatus.tsx
│   │
│   ├── determinism/
│   │   ├── DeterminismPanel.tsx
│   │   ├── DigestTable.tsx
│   │   └── VerificationStatus.tsx
│   │
│   ├── basketball/
│   │   ├── Bracket.tsx
│   │   ├── Matchup.tsx
│   │   ├── WinProbability.tsx
│   │   └── SeriesDistribution.tsx
│   │
│   ├── benchmarks/
│   │   ├── BenchmarkPanel.tsx
│   │   ├── SpeedupChart.tsx
│   │   └── MetricsTable.tsx
│   │
│   └── logs/
│       └── EngineLog.tsx
│
├── stores/
│   └── engineStore.ts
│
├── lib/
│   ├── tauri.ts
│   └── formatting.ts
│
└── styles/
    └── tokens.css
```

Adapt this to the existing repository rather than restructuring the entire project unnecessarily.

---

# 26. Design Tokens

Create a centralized token system.

Conceptually:

```css
:root {
  --bg-primary: #0A0B0D;
  --bg-secondary: #111317;
  --bg-panel: #15181D;

  --border: #2A2D33;
  --border-subtle: #1C1E22;

  --text-primary: #E6E8EB;
  --text-secondary: #969BA3;
  --text-muted: #626872;

  --accent: #3B82F6;

  --success: ...;
  --error: ...;

  --font-ui: ...;
  --font-mono: ...;
}
```

Do not scatter arbitrary colors through components.

---

# 27. Border / Radius Rules

Avoid the modern SaaS-card look.

Recommended:
- small or zero corner radius
- thin borders
- subtle separators
- no large shadows

The UI should feel like a technical instrument, not a collection of floating cards.

---

# 28. Responsive Behavior

This is primarily a desktop application.

Prioritize:
1. 1080p desktop
2. 1440p desktop
3. larger displays

Do not spend significant development time on mobile layouts.

At smaller desktop widths:
- reduce panel density
- allow panels to stack
- preserve critical information
- keep controls accessible

---

# 29. Accessibility

Even though this is a technical tool:
- use readable contrast
- never communicate pass/fail only through color
- provide text labels for statuses
- support keyboard controls
- use semantic buttons/inputs
- keep focus states visible
- do not make critical controls tiny

Use:

```text
✓ VERIFIED
```

rather than only a green square.

---

# 30. Error States

Errors must be explicit.

Example:

```text
ENGINE ERROR

Simulation failed to start.

Reason:
invalid thread count: 128
```

Determinism failure:

```text
DETERMINISM FAILURE

Seed: 42
Simulations: 1,000,000

Expected:
91d4c...a82

Received at 32 threads:
7a13f...991
```

Never silently fail.

Never display stale successful results as though they came from the latest failed run.

---

# 31. Loading / Empty States

Before a simulation:

```text
NO RUN DATA

Configure the engine and press RUN SIMULATION.
```

During determinism verification:

```text
VERIFYING

1 / 6 configurations complete
```

After completion:

```text
RUN COMPLETE

1,000,000 simulations
3.188 s
313,674 games/s
```

These example numbers are presentation examples only and must not be hardcoded.

---

# 32. Remove From the Old UI

If the existing prototype contains any of these, remove them:
- JavaScript simulation engines
- `setInterval()` simulation loops
- fake weather systems
- fake gate queues
- fake crowd simulation
- mock medical/security events
- fake CPU metrics
- simulated throughput counters
- arbitrary dashboard KPIs
- LLM-generated operational actions
- decorative AI chat interfaces
- fake real-time activity

The new application must make it impossible to confuse frontend animation with engine computation.

---

# 33. Do Not Build Yet

Do not spend time on:
- authentication
- cloud deployment
- multiplayer
- WebSockets unless genuinely required
- user accounts
- database infrastructure
- notification systems
- elaborate theme switching
- mobile support
- elaborate 3D basketball graphics
- animated basketball physics
- AI chatbot functionality

These distract from the systems thesis.

---

# 34. Implementation Order

## Phase 1 — Audit

Before changing the frontend:
1. inspect the existing Tauri project;
2. inspect the existing React structure;
3. identify old simulation/mock logic;
4. identify current Rust commands;
5. identify current engine result types;
6. identify current build scripts;
7. determine the cleanest integration boundary.

Do not rewrite working engine code merely to match this document.

Produce a short architecture summary before major UI changes.

---

## Phase 2 — Visual Shell

Build:
- application shell
- header
- control bar
- panel grid
- typography
- design tokens
- borders
- spacing
- dark theme

Use clearly labeled empty states.

Do not fake data.

---

## Phase 3 — Real Tauri IPC

Implement:
- run command
- configuration serialization
- result serialization
- error propagation

Test:

```text
React control
    ↓
Tauri
    ↓
Rust
    ↓
simulation
    ↓
Rust result
    ↓
React
```

The end-to-end path must work before visualization polish.

---

## Phase 4 — Streaming Telemetry

Implement:
- progress event
- completed count
- elapsed time
- throughput
- worker utilization if available
- run state

Target approximately 10 Hz UI updates.

Verify telemetry does not materially distort benchmark mode.

---

## Phase 5 — Determinism Console

Implement this before the bracket.

Required:
- seed display
- simulation count
- thread configurations
- digest table
- comparison
- pass/fail state
- mismatch details

This is the highest-signal UI feature.

---

## Phase 6 — Throughput Panel

Connect:
- games/sec
- elapsed time
- completed simulations
- worker count
- worker utilization

All values must come from the engine.

---

## Phase 7 — Basketball Visualization

Connect real simulation output.

Implement:
1. bracket
2. matchup probabilities
3. advancement probabilities
4. series-length distributions if supported
5. score distributions if supported

Do not implement visualizations for data the engine does not actually calculate.

---

## Phase 8 — Benchmark Panel

Start with imported benchmark results.

Display:
- thread count
- throughput
- speedup
- efficiency
- benchmark name
- sample/run information where available

Add Amdahl visualization after the raw data table works.

---

## Phase 9 — Logs

Connect real engine events.

Add:
- timestamps
- severity
- event type
- bounded buffer
- autoscroll behavior

---

## Phase 10 — Polish

Only after all real data paths work:
- spacing refinement
- typography refinement
- border refinement
- chart styling
- density tuning
- keyboard accessibility
- error-state polish
- performance tuning

Do not polish placeholder components for hours before wiring the backend.

---

# 35. Testing Requirements

## IPC tests

Verify:
- valid run
- invalid configuration
- engine failure
- cancellation if implemented
- large simulation counts

## Determinism UI tests

Verify that multiple thread configurations render as verified when their real digests match.

Also test a deliberate mismatch.

## Telemetry tests

Verify:
- progress monotonically increases
- completed <= total
- final completed == total
- throughput is derived from actual counters
- UI remains responsive

## Frontend tests

Test:
- control validation
- run state transitions
- error rendering
- determinism status
- empty states

---

# 36. UI Performance Requirements

The UI must not become the bottleneck.

Requirements:
- throttle progress updates
- avoid rendering huge log histories
- avoid unnecessary React re-renders
- keep chart datasets bounded
- do not serialize massive simulation structures through IPC
- send aggregates rather than raw per-game data
- use virtualization if a large table is genuinely required

Critical rule:

> The engine benchmark must be able to run without the UI and without telemetry overhead.

There must remain a benchmark/CLI path independent of Tauri.

---

# 37. Data Flow Rules

The following ownership model is mandatory:

```text
                    SOURCE OF TRUTH

Rust engine
    │
    ├── simulation results
    ├── deterministic digest
    ├── progress
    ├── throughput
    ├── worker telemetry
    ├── benchmark results
    └── engine logs
             │
             ▼
         Tauri IPC
             │
             ▼
       Zustand / React
             │
             ▼
          rendering
```

Never:

```text
React
  ↓
invent fake telemetry
  ↓
display as engine data
```

Never:

```text
React
  ↓
run a second basketball simulation
```

---

# 38. Interview-Oriented Design

The UI should allow a technical interviewer to understand the project's thesis in roughly 30 seconds.

They should be able to see:
1. native engine
2. configurable thread count
3. high-throughput simulation
4. real worker telemetry
5. deterministic output
6. same SHA-256 across thread counts
7. measured scaling
8. actual basketball simulation results

The strongest interaction is:

```text
Seed: 42
Simulations: 1,000,000

Threads:
1  → digest A
2  → digest A
4  → digest A
8  → digest A
16 → digest A
32 → digest A

✓ DETERMINISTIC
```

That is stronger evidence than a polished dashboard full of arbitrary statistics.

---

# 39. Visual QA Checklist

## Visual
- [ ] Near-black background
- [ ] Dense information hierarchy
- [ ] No decorative gradients
- [ ] No excessive rounded cards
- [ ] No giant hero KPI
- [ ] No unnecessary shadows
- [ ] Monospace numerical output
- [ ] Restrained color usage
- [ ] Green only means verified/pass
- [ ] Red only means failure/mismatch
- [ ] One primary accent
- [ ] Consistent borders
- [ ] Consistent spacing

## Functional
- [ ] Seed controls real engine seed
- [ ] Simulation count controls real engine workload
- [ ] Thread count controls real execution
- [ ] Run button launches real engine
- [ ] Progress comes from Rust
- [ ] Throughput comes from Rust
- [ ] Worker telemetry comes from Rust
- [ ] Determinism uses real engine digests
- [ ] Basketball probabilities come from real simulation
- [ ] Benchmark panel uses measured data
- [ ] Logs come from real engine events
- [ ] Errors are surfaced

## Systems credibility
- [ ] No frontend simulation loop
- [ ] No fake CPU usage
- [ ] No fake throughput
- [ ] No fake progress
- [ ] No mock benchmark data in production UI
- [ ] UI can be disabled for benchmark measurements
- [ ] CLI/native engine remains independently runnable

---

# 40. Definition of Done

The UI is complete when:
1. A user can configure a real basketball simulation from the desktop application.
2. Configuration reaches the Rust engine through Tauri IPC.
3. The engine executes the requested simulation.
4. The frontend receives real progress telemetry.
5. Throughput is calculated from real engine work.
6. Worker telemetry reflects real backend measurements.
7. Determinism verification can execute multiple thread configurations.
8. The UI displays resulting SHA-256 digests.
9. Identical outputs are clearly marked as verified.
10. Mismatches are clearly exposed.
11. Basketball tournament/matchup results are rendered from real simulation output.
12. Benchmark results can be displayed without fabricated values.
13. Engine logs reflect actual events.
14. No old frontend mock simulation remains.
15. The application looks like technical instrumentation rather than a generic sports dashboard.
16. The engine can still be benchmarked independently of the UI.

---

# 41. Instructions to Antigravity

You are implementing a UI around an existing native Rust basketball simulation engine.

Treat this document as a **design and implementation specification**, not an invitation to invent new product features.

### Non-negotiable rules

1. **Inspect before modifying.** Understand the existing repository and engine architecture first.
2. **Do not rewrite the engine unnecessarily.** Integrate with existing Rust APIs where possible.
3. **Remove old fake simulation logic.** The frontend must not simulate engine behavior.
4. **Never fabricate telemetry.** If the backend does not expose a metric, wire the backend to expose it or omit the visualization.
5. **Never fabricate benchmark numbers.** Use real measurements or explicit unavailable/empty states.
6. **Rust is authoritative.** React renders engine state; it does not reproduce it.
7. **Build the determinism console early.** It is the highest-value UI feature.
8. **Keep the UI dense and technical.** Avoid generic SaaS/dashboard aesthetics.
9. **Use animation only for real state changes.**
10. **Do not add unnecessary dependencies.** Prefer existing dependencies and lightweight implementations.
11. **Do not overengineer the UI.** The purpose is to expose the engine, not create a second project.
12. **Keep the engine independently benchmarkable.** Tauri/React must never be required to measure core engine performance.

### Recommended sequence

```text
AUDIT
  ↓
DESIGN TOKENS + SHELL
  ↓
TAURI IPC
  ↓
STREAMING TELEMETRY
  ↓
DETERMINISM CONSOLE
  ↓
THROUGHPUT / WORKER TELEMETRY
  ↓
BASKETBALL RESULTS
  ↓
BENCHMARKS
  ↓
ENGINE LOG
  ↓
POLISH
```

At each stage:

1. implement;
2. build/run;
3. test;
4. inspect visually;
5. verify displayed data is real;
6. only then proceed.

Do not implement every panel with placeholder data and call the project complete.

---

# 42. Final Design Thesis

The finished application should feel like:

> **A control and instrumentation console for a deterministic, high-throughput native basketball simulation engine.**

The basketball visualization makes the workload understandable.

The telemetry demonstrates that the engine is actually running.

The determinism console demonstrates reproducibility across thread counts.

The benchmark panel demonstrates measured scaling.

The UI's job is not to impress through decoration.

Its job is to make the **systems engineering impossible to miss**.
