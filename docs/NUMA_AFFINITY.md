# NUMA Awareness & CPU Core Affinity Analysis

## 1. Executive Summary & Systems Context

In multi-threaded high-throughput simulation engines, thread placement and memory topology significantly influence CPU cache efficiency and interconnect bandwidth.

This document investigates the impact of **Non-Uniform Memory Access (NUMA)** architecture and **CPU Core Affinity (Thread Pinning)** on the `NBA-2026-Postseason-Engine`.

---

## 2. Theoretical Architecture: NUMA vs. UMA

### Uniform Memory Access (UMA) — Desktop / Single-Socket
* **Architecture**: All CPU cores connect to memory controllers via a shared system bus / memory crossbar. Access latency to any RAM address is uniform across all cores.
* **Cache Topology**: Cores possess dedicated L1/L2 caches and share a monolithic or CCX-partitioned L3 cache.

### Non-Uniform Memory Access (NUMA) — Multi-Socket Servers
* **Architecture**: Physical memory is split across multiple CPU sockets/nodes. Each socket has its own dedicated local memory controller.
* **Interconnect Latency**: Accessing local memory takes ~80–100ns; accessing remote memory across the socket interconnect (Intel UPI / AMD Infinity Fabric) takes ~200–300ns.
* **Systems Implication**: On NUMA systems, allocating memory and pinning worker threads to the *same* physical socket is mandatory to avoid cross-socket bus saturation.

---

## 3. Implementation of CPU Affinity (`core_affinity`)

We integrated cross-platform core affinity into both our **Rayon** parallel runtime and our **Custom Chase-Lev Work-Stealing Scheduler** using the `core_affinity` crate.

### Rayon Worker Thread Pinning
```rust
let mut builder = rayon::ThreadPoolBuilder::new().num_threads(args.threads as usize);
if args.pin_threads {
    if let Some(core_ids) = core_affinity::get_core_ids() {
        builder = builder.start_handler(move |idx| {
            if let Some(core_id) = core_ids.get(idx % core_ids.len()) {
                core_affinity::set_for_current(*core_id);
            }
        });
    }
}
let pool = builder.build().unwrap();
```

### Custom Work-Stealing Worker Pinning
```rust
let core_ids = if pin_threads { core_affinity::get_core_ids() } else { None };
for (i, worker) in workers.into_iter().enumerate() {
    let core_id = core_ids.as_ref().and_then(|ids| ids.get(i % ids.len()).copied());
    let handle = s.spawn(move |_| {
        if let Some(id) = core_id {
            core_affinity::set_for_current(id);
        }
        // ... simulation loop ...
    });
}
```

---

## 4. Empirical Benchmark Results

Benchmarked on a 16-thread system across **100,000 full postseason simulations** (8,889,182 individual postseason games) with seed `42`:

| Scheduler | Pinning Mode | Runtime (s) | Postseason Throughput | Game Throughput | Delta vs Baseline |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Rayon** | **Unpinned (OS Scheduled)** | **4.1721s** | **23,969 ps/s** | **2,130,610 games/s** | **Baseline (100%)** |
| **Rayon** | **Pinned (`--pin-threads`)** | **4.3509s** | **22,983 ps/s** | **2,043,045 games/s** | **-4.1% Throughput** |
| **Custom Work-Stealing** | **Unpinned (OS Scheduled)** | **4.2064s** | **23,773 ps/s** | **2,113,238 games/s** | **-0.8% Throughput** |
| **Custom Work-Stealing** | **Pinned (`--pin-threads`)** | **4.3339s** | **23,074 ps/s** | **2,051,097 games/s** | **-3.7% Throughput** |

---

## 5. Low-Level Systems Analysis: Why Pinning Slowed Down Desktop Execution

### 1. SMT / Hyperthreading Resource Contention
On desktop processors with Simultaneous Multithreading (SMT / Hyperthreading), logical core indices `2n` and `2n+1` map to the same physical core, sharing execution ports and L1/L2 caches. Hard-pinning 16 threads sequentially binds pairs of saturated worker threads to shared execution pipelines, eliminating the OS scheduler's ability to balance workloads across under-utilized physical execution units.

### 2. OS Context-Switch Friction & Thermal Balancing
Modern OS schedulers dynamically balance CPU loads across cores to manage core temperatures and handle background OS tasks. When a thread is hard-pinned to a core that the OS needs for a background interrupt or DPC, the worker thread is forced to wait on that exact core rather than migrating instantaneously to an idle pipeline.

### 3. Hot-Path Zero-Allocation Fits in L1/L2 Cache
Because our engine uses zero heap allocations inside the simulation hot path, the working set of a postseason simulation is only ~2–4 KB (register and stack-resident). Thread migration penalties are virtually unnoticeable because rebuilding the working set in L1/L2 cache takes fewer than 100 CPU cycles.

---

## 6. Recommendations & Conclusions

1. **Desktop / UMA Environments**: Keep `--pin-threads` disabled by default. Let the OS scheduler and Rayon dynamic work-stealing handle core balancing.
2. **Dedicated Multi-Socket HPC / Cloud Deployments**: On bare-metal servers with multi-socket NUMA nodes (e.g., dual-socket AWS c6i.metal or AMD EPYC), enable `--pin-threads` combined with NUMA memory binding (`numactl --interleave=all` or socket-isolated process instances) to prevent UPI interconnect cross-talk.
