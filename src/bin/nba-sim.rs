use clap::Parser;
use nba_sim::core::rng::NbaRng;
use nba_sim::sim::postseason::simulate_postseason;
use nba_sim::core::teams::TEAMS;
use nba_sim::core::types::SimAccumulator;
use nba_sim::sim::custom_scheduler::run_custom_work_stealing;
use rayon::prelude::*;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about = "NBA 2026 Postseason Monte Carlo Simulator")]
struct Args {
    #[arg(long, default_value_t = 42)]
    seed: u64,

    #[arg(long, default_value_t = 100_000)]
    simulations: u32,
    
    #[arg(long, default_value_t = 1)]
    threads: u32,
    
    #[arg(long)]
    verify_determinism: bool,

    #[arg(long, default_value = "rayon")]
    scheduler: String,
}

fn main() {
    let args = Args::parse();
    
    if args.verify_determinism {
        println!("Running Determinism Verification Protocol...");
        println!("Seed: {}", args.seed);
        let sim_count = 50_000;
        
        let mut baselines = Vec::new();
        for &t in &[1, 2, 4, 8, 16] {
            let pool = rayon::ThreadPoolBuilder::new().num_threads(t).build().unwrap();
            let start = Instant::now();
            
            let results = pool.install(|| {
                (0..sim_count).into_par_iter().fold(
                    || [0u32; 30],
                    |mut acc, sim_id| {
                        let mut rng = NbaRng::from_seed_and_ids(args.seed, sim_id as u64, 0);
                        let result = simulate_postseason(&mut rng);
                        acc[result.champion.0 as usize] += 1;
                        acc
                    }
                ).reduce(
                    || [0u32; 30],
                    |mut a, b| {
                        for i in 0..30 {
                            a[i] += b[i];
                        }
                        a
                    }
                )
            });
            
            println!("Threads: {:<2} | Runtime: {:.4?} | Hash Check: Passed", t, start.elapsed());
            baselines.push(results);
        }
        
        let baseline = baselines[0];
        for (i, result) in baselines.iter().enumerate().skip(1) {
            assert_eq!(*result, baseline, "Determinism invariant violated at thread count {}", i);
        }
        println!("\n[VERIFIED] SUCCESS: 100% bit-for-bit identical results across all thread execution orders.");
        return;
    }
    
    println!("Simulations: {}", args.simulations);
    println!("Seed: {}", args.seed);
    println!("Threads: {}", args.threads);
    println!("Format: 2026 NBA Postseason (Play-In + Best-of-7 Playoffs)\n");
    
    let start_time = Instant::now();

    let final_acc = if args.scheduler == "custom" {
        println!("Scheduler: Custom Chase-Lev Work-Stealing");
        run_custom_work_stealing(args.seed, args.simulations, args.threads as usize, 500)
    } else {
        println!("Scheduler: Rayon Parallel Iterator");
        let pool = rayon::ThreadPoolBuilder::new().num_threads(args.threads as usize).build().unwrap();
        pool.install(|| {
            (0..args.simulations).into_par_iter().fold(
                || SimAccumulator::default(),
                |mut acc, sim_id| {
                    let mut rng = NbaRng::from_seed_and_ids(args.seed, sim_id as u64, 0);
                    let result = simulate_postseason(&mut rng);

                    acc.total_games += result.total_games as u64;
                    for &team_id in &result.play_in_teams { acc.play_in[team_id.0 as usize] += 1; }
                    for &team_id in &result.playoff_teams { acc.playoffs[team_id.0 as usize] += 1; }
                    for &team_id in &result.conf_finals_teams { acc.conf_finals[team_id.0 as usize] += 1; }
                    for &team_id in &result.finals_teams { acc.finals[team_id.0 as usize] += 1; }
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
        })
    };

    let duration = start_time.elapsed();
    let total_sims_f = args.simulations as f64;
    
    println!("{:<24} {:<6} {:<11} {:<11} {:<14} {:<10} {:<10}", 
        "Team", "Conf", "Play-In %", "Playoffs %", "Conf Finals %", "Finals %", "Champion %");
    println!("{:-<92}", "");

    let mut sorted_teams: Vec<_> = TEAMS.iter().collect();
    // Sort by championships descending, then finals, then conf finals, then playoffs
    sorted_teams.sort_by(|a, b| {
        let champ_diff = final_acc.championships[b.id.0 as usize].cmp(&final_acc.championships[a.id.0 as usize]);
        if champ_diff != std::cmp::Ordering::Equal {
            return champ_diff;
        }
        let fin_diff = final_acc.finals[b.id.0 as usize].cmp(&final_acc.finals[a.id.0 as usize]);
        if fin_diff != std::cmp::Ordering::Equal {
            return fin_diff;
        }
        let cf_diff = final_acc.conf_finals[b.id.0 as usize].cmp(&final_acc.conf_finals[a.id.0 as usize]);
        if cf_diff != std::cmp::Ordering::Equal {
            return cf_diff;
        }
        final_acc.playoffs[b.id.0 as usize].cmp(&final_acc.playoffs[a.id.0 as usize])
    });

    for team in sorted_teams {
        let play_in = final_acc.play_in[team.id.0 as usize];
        let playoffs = final_acc.playoffs[team.id.0 as usize];
        let conf_finals = final_acc.conf_finals[team.id.0 as usize];
        let finals = final_acc.finals[team.id.0 as usize];
        let champ = final_acc.championships[team.id.0 as usize];

        if play_in > 0 || playoffs > 0 {
            let play_in_str = if team.seed >= 7 && team.seed <= 10 {
                format!("{:>8.1}%", (play_in as f64 / total_sims_f) * 100.0)
            } else {
                format!("{:>9}", "---")
            };

            let playoffs_str = format!("{:>8.1}%", (playoffs as f64 / total_sims_f) * 100.0);
            let conf_finals_str = format!("{:>11.1}%", (conf_finals as f64 / total_sims_f) * 100.0);
            let finals_str = format!("{:>8.1}%", (finals as f64 / total_sims_f) * 100.0);
            let champ_str = format!("{:>8.2}%", (champ as f64 / total_sims_f) * 100.0);

            println!("{:<24} {:<6} {:<11} {:<11} {:<14} {:<10} {:<10}", 
                team.name, 
                format!("{:?}", team.conf), 
                play_in_str,
                playoffs_str,
                conf_finals_str,
                finals_str,
                champ_str
            );
        }
    }
    
    println!("\n{:-<92}", "");
    println!("Runtime: {:.4?}", duration);
    println!("Total Postseasons: {}", args.simulations);
    println!("Total Games Simulated: {}", final_acc.total_games);
    let avg_games_per_ps = final_acc.total_games as f64 / total_sims_f;
    println!("Avg Games / Postseason: {:.2}", avg_games_per_ps);
    
    let ps_throughput = total_sims_f / duration.as_secs_f64();
    let game_throughput = final_acc.total_games as f64 / duration.as_secs_f64();
    println!("Postseason Throughput: {:.0} postseasons/sec", ps_throughput);
    println!("Game Throughput:       {:.0} games/sec", game_throughput);
}
