use clap::Parser;
use nba_sim::core::rng::NbaRng;
use nba_sim::sim::postseason::simulate_postseason;
use nba_sim::core::teams::TEAMS;
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
}

fn main() {
    let args = Args::parse();
    
    println!("Simulations: {}", args.simulations);
    println!("Seed: {}", args.seed);
    println!("Threads: {}", args.threads);
    println!("Format: 2026 NBA Postseason (Play-In + Best-of-7 Playoffs)\n");
    
    let start_time = Instant::now();
    let mut championships = [0u32; 30];

    // Single-threaded simulation loop for Phase 1
    for sim_id in 0..args.simulations {
        // Hierarchical seeded RNG for deterministic execution (SimID changes per loop)
        let mut rng = NbaRng::from_seed_and_ids(args.seed, sim_id as u64, 0);
        let result = simulate_postseason(&mut rng);
        championships[result.champion.0 as usize] += 1;
    }

    let duration = start_time.elapsed();
    
    println!("{:<22} {:<7} {:<10}", "Team", "Conf", "Champion %");

    let mut sorted_teams: Vec<_> = TEAMS.iter().collect();
    // Sort by championships descending
    sorted_teams.sort_by(|a, b| {
        let wins_a = championships[a.id.0 as usize];
        let wins_b = championships[b.id.0 as usize];
        wins_b.cmp(&wins_a)
    });

    for team in sorted_teams {
        let wins = championships[team.id.0 as usize];
        if wins > 0 {
            let champ_pct = (wins as f64 / args.simulations as f64) * 100.0;
            println!("{:<22} {:<7} {:<10.2}%", 
                team.name, 
                format!("{:?}", team.conf), 
                champ_pct
            );
        }
    }
    
    println!("\nRuntime: {:.2?}", duration);
    let throughput = args.simulations as f64 / duration.as_secs_f64();
    println!("Throughput: {:.0} postseasons/sec", throughput);
}
