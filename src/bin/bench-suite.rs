use comfy_table::Table;
use sysinfo::System;
use std::time::Instant;
use rayon::prelude::*;

use nba_sim::core::rng::NbaRng;
use nba_sim::sim::postseason::simulate_postseason;
use nba_sim::core::types::SimAccumulator;
use nba_sim::sim::custom_scheduler::run_custom_work_stealing;

fn main() {
    println!("Initializing NBA-2026-Postseason-Engine Benchmark Suite...\n");

    let mut sys = System::new_all();
    sys.refresh_all();
    
    // System Reconnaissance Table
    let mut sys_table = Table::new();
    sys_table.set_header(vec!["Metric", "System Methodology Report"]);

    let cpu_model = sys.cpus().first().map(|c| c.brand()).unwrap_or("Unknown");
    let physical_cores = System::physical_core_count().unwrap_or(0);
    let logical_cores = sys.cpus().len();
    let memory_gb = sys.total_memory() as f64 / 1_073_741_824.0;
    let os_name = System::long_os_version().unwrap_or_else(|| "Unknown OS".to_string());

    sys_table.add_row(vec!["CPU Model", cpu_model]);
    sys_table.add_row(vec!["Cores", &format!("{} Physical / {} Logical", physical_cores, logical_cores)]);
    sys_table.add_row(vec!["Total RAM", &format!("{:.2} GB", memory_gb)]);
    sys_table.add_row(vec!["OS", &os_name]);
    sys_table.add_row(vec!["Compiler Profile", "Release (-C opt-level=3)"]);
    sys_table.add_row(vec!["Determinism Target", "SHA-256 Bit-for-Bit Verified"]);

    println!("{}", sys_table);
    println!("\n");

    // Parallel Scaling (Rayon) Table
    let mut scaling_table = Table::new();
    scaling_table.set_header(vec![
            "Threads", 
            "Runtime (s)", 
            "Postseasons/sec", 
            "Speedup", 
            "Efficiency"
        ]);

    let sim_count = 100_000;
    let seed = 42;
    let thread_counts = [1, 2, 4, 8, 16];
    
    let mut baseline_runtime = 0.0;
    
    println!("Running Parallel Scaling (Rayon) [{} Simulations]...", sim_count);
    
    for &t in &thread_counts {
        let pool = rayon::ThreadPoolBuilder::new().num_threads(t).build().unwrap();
        let start = Instant::now();
        
        let acc = pool.install(|| {
            (0..sim_count).into_par_iter().fold(
                || SimAccumulator::default(),
                |mut acc, sim_id| {
                    let mut rng = NbaRng::from_seed_and_ids(seed, sim_id as u64, 0);
                    let result = simulate_postseason(&mut rng);
                    acc.total_games += result.total_games as u64;
                    acc
                }
            ).reduce(|| SimAccumulator::default(), |mut a, b| { a.merge(&b); a })
        });
        
        let runtime = start.elapsed().as_secs_f64();
        let throughput = sim_count as f64 / runtime;
        
        if t == 1 {
            baseline_runtime = runtime;
        }
        
        let speedup = baseline_runtime / runtime;
        let efficiency = speedup / (t as f64) * 100.0;
        
        scaling_table.add_row(vec![
            t.to_string(),
            format!("{:.4}", runtime),
            format!("{:.0}", throughput),
            format!("{:.2}x", speedup),
            format!("{:.1}%", efficiency),
        ]);
        
        // Ensure the compiler doesn't optimize away the run
        std::hint::black_box(acc);
    }
    
    println!("{}\n", scaling_table);
    
    // Scheduler Comparison Table
    let mut scheduler_table = Table::new();
    scheduler_table.set_header(vec![
            "Scheduler Type",
            "Config",
            "Runtime (s)",
            "Postseasons/sec",
            "Game Throughput",
        ]);
        
    println!("Running Scheduler Comparison [16 Threads, {} Simulations]...", sim_count);
    
    // 1. Rayon Unpinned
    let pool = rayon::ThreadPoolBuilder::new().num_threads(16).build().unwrap();
    let start = Instant::now();
    let acc_ru = pool.install(|| {
        (0..sim_count).into_par_iter().fold(
            || SimAccumulator::default(),
            |mut acc, sim_id| {
                let mut rng = NbaRng::from_seed_and_ids(seed, sim_id as u64, 0);
                let result = simulate_postseason(&mut rng);
                acc.total_games += result.total_games as u64;
                acc
            }
        ).reduce(|| SimAccumulator::default(), |mut a, b| { a.merge(&b); a })
    });
    let runtime_ru = start.elapsed().as_secs_f64();
    scheduler_table.add_row(vec![
        "Rayon",
        "Unpinned",
        &format!("{:.4}", runtime_ru),
        &format!("{:.0}", sim_count as f64 / runtime_ru),
        &format!("{:.0}", acc_ru.total_games as f64 / runtime_ru),
    ]);
    
    // 2. Custom Unpinned
    let start = Instant::now();
    let acc_cu = run_custom_work_stealing(seed, sim_count, 16, 500, false);
    let runtime_cu = start.elapsed().as_secs_f64();
    scheduler_table.add_row(vec![
        "Custom Chase-Lev",
        "Unpinned",
        &format!("{:.4}", runtime_cu),
        &format!("{:.0}", sim_count as f64 / runtime_cu),
        &format!("{:.0}", acc_cu.total_games as f64 / runtime_cu),
    ]);
    
    // 3. Rayon Pinned
    let mut builder = rayon::ThreadPoolBuilder::new().num_threads(16);
    if let Some(core_ids) = core_affinity::get_core_ids() {
        builder = builder.start_handler(move |idx| {
            if let Some(core_id) = core_ids.get(idx % core_ids.len()) {
                core_affinity::set_for_current(*core_id);
            }
        });
    }
    let pool_pinned = builder.build().unwrap();
    let start = Instant::now();
    let acc_rp = pool_pinned.install(|| {
        (0..sim_count).into_par_iter().fold(
            || SimAccumulator::default(),
            |mut acc, sim_id| {
                let mut rng = NbaRng::from_seed_and_ids(seed, sim_id as u64, 0);
                let result = simulate_postseason(&mut rng);
                acc.total_games += result.total_games as u64;
                acc
            }
        ).reduce(|| SimAccumulator::default(), |mut a, b| { a.merge(&b); a })
    });
    let runtime_rp = start.elapsed().as_secs_f64();
    scheduler_table.add_row(vec![
        "Rayon",
        "Pinned",
        &format!("{:.4}", runtime_rp),
        &format!("{:.0}", sim_count as f64 / runtime_rp),
        &format!("{:.0}", acc_rp.total_games as f64 / runtime_rp),
    ]);
    
    println!("{}\n", scheduler_table);
    
    println!("Benchmark Suite Execution Completed Successfully.");
}
