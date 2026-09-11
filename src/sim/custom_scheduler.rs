use crate::core::rng::NbaRng;
use crate::core::types::SimAccumulator;
use crate::sim::postseason::simulate_postseason;
use crossbeam_deque::{Steal, Worker};
use crossbeam_utils::{thread, Backoff};

/// Run Monte Carlo simulations using a custom Chase-Lev work-stealing scheduler with exponential backoff.
pub fn run_custom_work_stealing(
    seed: u64,
    total_simulations: u32,
    num_threads: usize,
    _chunk_size: u32, // Unused now, we use native stealing
) -> SimAccumulator {
    // 1. Create a queue for each thread
    let workers: Vec<_> = (0..num_threads).map(|_| Worker::new_fifo()).collect();
    
    // 2. Distribute `Stealer` handles
    let stealers: Vec<_> = workers.iter().map(|w| w.stealer()).collect();

    // 3. Partition tasks into much smaller chunks (e.g. 50) to allow fine-grained native stealing.
    // We push them into the first worker's queue, and let the stealers load-balance naturally.
    // Or we round-robin them. Let's round-robin chunks of 50.
    let base_chunk = 50;
    let mut current_sim = 0;
    let mut worker_idx = 0;
    
    while current_sim < total_simulations {
        let end = (current_sim + base_chunk).min(total_simulations);
        workers[worker_idx].push(current_sim..end);
        current_sim = end;
        worker_idx = (worker_idx + 1) % num_threads;
    }

    // 4. Spawn scoped threads
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

                    // If local queue is empty, attempt to steal from other threads with exponential backoff
                    let backoff = Backoff::new();
                    let mut stolen = false;
                    let steal_offset = (i + 1) % stealers.len();
                    
                    while !stolen {
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
                                    // Someone else is stealing, yield quickly
                                    backoff.snooze();
                                }
                                Steal::Empty => continue,
                            }
                        }

                        if stolen {
                            break;
                        }

                        // If we did a full sweep and found nothing, check if we should keep trying
                        if backoff.is_completed() {
                            // After backing off maximally (which includes yielding to the OS), we assume all queues are completely empty.
                            break;
                        }
                        backoff.snooze();
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
