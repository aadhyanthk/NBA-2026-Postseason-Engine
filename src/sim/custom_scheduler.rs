use crate::core::rng::NbaRng;
use crate::core::types::SimAccumulator;
use crate::sim::postseason::simulate_postseason;
use crossbeam_deque::{Steal, Worker};
use crossbeam_utils::thread;
use std::iter;

/// Run Monte Carlo simulations using a custom Chase-Lev work-stealing scheduler.
pub fn run_custom_work_stealing(
    seed: u64,
    total_simulations: u32,
    num_threads: usize,
    chunk_size: u32,
) -> SimAccumulator {
    // 1. Create a queue for each thread
    let workers: Vec<_> = (0..num_threads).map(|_| Worker::new_fifo()).collect();
    
    // 2. Distribute `Stealer` handles so threads can steal from each other
    let stealers: Vec<_> = workers.iter().map(|w| w.stealer()).collect();

    // 3. Partition tasks into chunks
    let mut current_sim = 0;
    let mut worker_idx = 0;
    
    while current_sim < total_simulations {
        let end = (current_sim + chunk_size).min(total_simulations);
        let chunk = current_sim..end;
        
        workers[worker_idx].push(chunk);
        
        current_sim = end;
        worker_idx = (worker_idx + 1) % num_threads;
    }

    // 4. Spawn scoped threads to process the queues
    thread::scope(|s| {
        let mut handles = Vec::with_capacity(num_threads);
        
        for (i, worker) in workers.into_iter().enumerate() {
            let stealers = stealers.clone();
            
            let handle = s.spawn(move |_| {
                let mut local_acc = SimAccumulator::default();
                
                loop {
                    // Try to pop a chunk from the local queue
                    if let Some(chunk) = worker.pop() {
                        for sim_id in chunk {
                            let mut rng = NbaRng::from_seed_and_ids(seed, sim_id as u64, 0);
                            let result = simulate_postseason(&mut rng);
                            
                            local_acc.total_games += result.total_games as u64;
                            for &team_id in &result.play_in_teams { local_acc.play_in[team_id.0 as usize] += 1; }
                            for &team_id in &result.playoff_teams { local_acc.playoffs[team_id.0 as usize] += 1; }
                            for &team_id in &result.conf_finals_teams { local_acc.conf_finals[team_id.0 as usize] += 1; }
                            for &team_id in &result.finals_teams { local_acc.finals[team_id.0 as usize] += 1; }
                            local_acc.championships[result.champion.0 as usize] += 1;
                        }
                        continue;
                    }

                    // If local queue is empty, attempt to steal from other threads
                    let mut stolen = false;
                    // Start stealing from a different offset to avoid contention
                    let steal_offset = (i + 1) % stealers.len();
                    
                    for j in 0..stealers.len() {
                        let target = (steal_offset + j) % stealers.len();
                        if target == i { continue; } // Don't steal from self

                        match stealers[target].steal_batch_and_pop(&worker) {
                            Steal::Success(chunk) => {
                                for sim_id in chunk {
                                    let mut rng = NbaRng::from_seed_and_ids(seed, sim_id as u64, 0);
                                    let result = simulate_postseason(&mut rng);
                                    
                                    local_acc.total_games += result.total_games as u64;
                                    for &team_id in &result.play_in_teams { local_acc.play_in[team_id.0 as usize] += 1; }
                                    for &team_id in &result.playoff_teams { local_acc.playoffs[team_id.0 as usize] += 1; }
                                    for &team_id in &result.conf_finals_teams { local_acc.conf_finals[team_id.0 as usize] += 1; }
                                    for &team_id in &result.finals_teams { local_acc.finals[team_id.0 as usize] += 1; }
                                    local_acc.championships[result.champion.0 as usize] += 1;
                                }
                                stolen = true;
                                break;
                            }
                            Steal::Retry => {
                                // Another thread was stealing at the same time, we could spin or retry
                                // For simplicity, we just continue attempting to steal from others
                                continue;
                            }
                            Steal::Empty => continue,
                        }
                    }

                    if !stolen {
                        // All queues are genuinely empty, terminate this thread
                        break;
                    }
                }
                
                local_acc
            });
            handles.push(handle);
        }
        
        // 5. Gather all local accumulators and reduce
        let mut final_acc = SimAccumulator::default();
        for handle in handles {
            let acc = handle.join().unwrap();
            final_acc.merge(&acc);
        }
        
        final_acc
    }).unwrap()
}
