use std::time::Instant;
use nba_sim::core::rng::NbaRng;
use nba_sim::core::seasons::SEASONS;
use nba_sim::core::teams::Team;
use nba_sim::core::types::SimAccumulator;
use nba_sim::sim::postseason::simulate_postseason;
use nba_sim::sim::custom_scheduler::run_custom_work_stealing;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tauri::{AppHandle, Emitter};

#[derive(Serialize, Clone)]
pub struct TelemetrySnapshot {
    pub completed: u32,
    pub total: u32,
    pub games_per_second: f64,
    pub elapsed_ns: u64,
    pub worker_utilization: Vec<f32>,
}

#[derive(Serialize)]
pub struct DeterminismRun {
    pub threads: u32,
    pub digest: String,
    pub elapsed_ns: u64,
    pub match_status: bool,
}

#[derive(Serialize)]
pub struct DeterminismResult {
    pub seed: u64,
    pub simulations: u32,
    pub runs: Vec<DeterminismRun>,
    pub all_match: bool,
}

#[tauri::command]
fn get_teams(year: Option<u32>) -> Vec<Team> {
    let y = year.unwrap_or(2026);
    let season = SEASONS.iter().find(|s| s.year == y).unwrap_or(&SEASONS[10]);
    season.teams.iter().cloned().collect()
}

#[tauri::command]
async fn run_simulation(
    app_handle: AppHandle,
    seed: u64,
    simulations: u32,
    threads: u32,
    scheduler: String,
    pin_threads: bool,
    year: Option<u32>,
) -> Result<SimAccumulator, String> {
    let start_time = Instant::now();
    let app_handle_clone = app_handle.clone();

    // Spawn blocking so we don't block the async runtime
    let final_acc = tauri::async_runtime::spawn_blocking(move || -> Result<SimAccumulator, String> {
        let y = year.unwrap_or(2026);
        let season = SEASONS.iter().find(|s| s.year == y).unwrap_or(&SEASONS[10]);
        if scheduler == "custom" {
            // Note: Custom scheduler doesn't easily emit progress without modifications,
            // for now just run it and return
            Ok(run_custom_work_stealing(seed, simulations, threads as usize, 500, pin_threads, season))
        } else {
            let mut builder = rayon::ThreadPoolBuilder::new().num_threads(threads as usize);
            if pin_threads {
                if let Some(core_ids) = core_affinity::get_core_ids() {
                    builder = builder.start_handler(move |idx| {
                        if let Some(core_id) = core_ids.get(idx % core_ids.len()) {
                            core_affinity::set_for_current(*core_id);
                        }
                    });
                }
            }
            let pool = builder.build().map_err(|e| e.to_string())?;

            let chunk_size = (simulations / 100).max(1);
            let update_interval = std::time::Duration::from_millis(100);
            let last_update = std::sync::Mutex::new(Instant::now());
            let completed_counter = std::sync::atomic::AtomicU32::new(0);
            let total_games_counter = std::sync::atomic::AtomicU64::new(0);

            let acc = pool.install(|| {
                (0..simulations).into_par_iter().fold(
                    || SimAccumulator::default(),
                    |mut acc, sim_id| {
                        let mut rng = NbaRng::from_seed_and_ids(seed, sim_id as u64, 0);
                        let result = simulate_postseason(&mut rng, season);

                        let games = result.total_games as u64;
                        acc.total_games += games;
                        let current_total = total_games_counter.fetch_add(games, std::sync::atomic::Ordering::Relaxed) + games;

                        for &team_id in &result.play_in_teams { acc.play_in[team_id.0 as usize] += 1; }
                        for &team_id in &result.playoff_teams { acc.playoffs[team_id.0 as usize] += 1; }
                        for &team_id in &result.conf_finals_teams { acc.conf_finals[team_id.0 as usize] += 1; }
                        for &team_id in &result.finals_teams { acc.finals[team_id.0 as usize] += 1; }
                        acc.championships[result.champion.0 as usize] += 1;

                        // Emit progress periodically
                        let completed = completed_counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
                        if completed % chunk_size == 0 || completed == simulations {
                            if let Ok(mut last) = last_update.try_lock() {
                                if last.elapsed() >= update_interval || completed == simulations {
                                    *last = Instant::now();
                                    let elapsed = start_time.elapsed();
                                    let elapsed_secs = elapsed.as_secs_f64();
                                    let games_per_second = if elapsed_secs > 0.0 {
                                        (current_total as f64) / elapsed_secs
                                    } else {
                                        0.0
                                    };
                                    
                                    let _ = app_handle_clone.emit("simulation://progress", TelemetrySnapshot {
                                        completed,
                                        total: simulations,
                                        games_per_second,
                                        elapsed_ns: elapsed.as_nanos() as u64,
                                        worker_utilization: vec![1.0; threads as usize], // Mock for now if we can't extract it easily
                                    });
                                }
                            }
                        }
                        acc
                    }
                ).reduce(
                    || SimAccumulator::default(),
                    |mut a, b| {
                        a.merge(&b);
                        a
                    }
                )
            });
            Ok(acc)
        }
    })
    .await
    .map_err(|e| e.to_string())??;

    let elapsed = start_time.elapsed();
    let _ = app_handle.emit("simulation://log", format!("COMPLETE: elapsed={:.3?} simulations={}", elapsed, simulations));

    Ok(final_acc)
}

#[tauri::command]
async fn verify_determinism(
    app_handle: AppHandle,
    seed: u64,
    simulations: u32,
    thread_counts: Vec<u32>,
) -> Result<DeterminismResult, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let mut runs = Vec::new();
        let mut baseline_hash = None;
        let mut all_match = true;

        let app_handle_clone = app_handle.clone();
        for &t in &thread_counts {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(t as usize)
                .build()
                .map_err(|e| e.to_string())?;

            let start = Instant::now();
            let results = pool.install(|| {
                (0..simulations).into_par_iter().fold(
                    || SimAccumulator::default(),
                    |mut acc, sim_id| {
                        let mut rng = NbaRng::from_seed_and_ids(seed, sim_id as u64, 0);
                        let result = simulate_postseason(&mut rng);
                        acc.total_games += result.total_games as u64;
                        acc.championships[result.champion.0 as usize] += 1;
                        acc
                    }
                ).reduce(
                    || SimAccumulator::default(),
                    |mut a, b| {
                        a.merge(&b);
                        a
                    }
                )
            });

            let elapsed_ns = start.elapsed().as_nanos() as u64;

            // Hash the results
            let mut hasher = Sha256::new();
            // Just hash the championships array for simplicity and speed of verification
            for &count in &results.championships {
                hasher.update(&count.to_le_bytes());
            }
            hasher.update(&results.total_games.to_le_bytes());
            let hash_bytes = hasher.finalize();
            let digest = hex::encode(hash_bytes);

            let mut match_status = true;
            if let Some(ref base) = baseline_hash {
                if base != &digest {
                    match_status = false;
                    all_match = false;
                }
            } else {
                baseline_hash = Some(digest.clone());
            }

            runs.push(DeterminismRun {
                threads: t,
                digest,
                elapsed_ns,
                match_status,
            });
            
            let _ = app_handle_clone.emit("simulation://log", format!("VERIFY [{}T]: digest={}", t, runs.last().unwrap().digest));
        }

        Ok(DeterminismResult {
            seed,
            simulations,
            runs,
            all_match,
        })
    }).await.map_err(|e| e.to_string())?
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_teams,
            run_simulation,
            verify_determinism
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
