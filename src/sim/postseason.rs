use crate::core::types::{Conference, TeamId};
use crate::core::rng::NbaRng;
use crate::core::teams::TEAMS;
use crate::sim::play_in::simulate_play_in;
use crate::sim::playoffs::simulate_series;

pub struct PostseasonResult {
    pub champion: TeamId,
}

pub fn simulate_postseason(rng: &mut NbaRng) -> PostseasonResult {
    // We use indices 1..=10 for seeds.
    let mut east_seeds = [TeamId(255); 11];
    let mut west_seeds = [TeamId(255); 11];

    // Load initial seeds
    for team in TEAMS.iter() {
        if team.seed > 0 && team.seed <= 10 {
            match team.conf {
                Conference::East => east_seeds[team.seed as usize] = team.id,
                Conference::West => west_seeds[team.seed as usize] = team.id,
            }
        }
    }

    // Play-in Tournament
    let (east_7, east_8) = simulate_play_in(east_seeds[7], east_seeds[8], east_seeds[9], east_seeds[10], rng);
    east_seeds[7] = east_7;
    east_seeds[8] = east_8;

    let (west_7, west_8) = simulate_play_in(west_seeds[7], west_seeds[8], west_seeds[9], west_seeds[10], rng);
    west_seeds[7] = west_7;
    west_seeds[8] = west_8;

    // Playoffs Round 1 (First Round)
    let east_r1_1 = simulate_series(east_seeds[1], east_seeds[8], rng).winner;
    let east_r1_4 = simulate_series(east_seeds[4], east_seeds[5], rng).winner;
    let east_r1_3 = simulate_series(east_seeds[3], east_seeds[6], rng).winner;
    let east_r1_2 = simulate_series(east_seeds[2], east_seeds[7], rng).winner;

    let west_r1_1 = simulate_series(west_seeds[1], west_seeds[8], rng).winner;
    let west_r1_4 = simulate_series(west_seeds[4], west_seeds[5], rng).winner;
    let west_r1_3 = simulate_series(west_seeds[3], west_seeds[6], rng).winner;
    let west_r1_2 = simulate_series(west_seeds[2], west_seeds[7], rng).winner;

    // Playoffs Round 2 (Conference Semifinals)
    // 1v8 winner vs 4v5 winner. By rule, highest remaining seed has home court advantage.
    // In our `simulate_series`, the first argument is home court. We don't dynamically reseed, 
    // but we should pass the higher original seed first for home court advantage.
    let east_sf_1 = simulate_series(get_higher_seed(east_r1_1, east_r1_4), get_lower_seed(east_r1_1, east_r1_4), rng).winner;
    let east_sf_2 = simulate_series(get_higher_seed(east_r1_2, east_r1_3), get_lower_seed(east_r1_2, east_r1_3), rng).winner;

    let west_sf_1 = simulate_series(get_higher_seed(west_r1_1, west_r1_4), get_lower_seed(west_r1_1, west_r1_4), rng).winner;
    let west_sf_2 = simulate_series(get_higher_seed(west_r1_2, west_r1_3), get_lower_seed(west_r1_2, west_r1_3), rng).winner;

    // Playoffs Round 3 (Conference Finals)
    let east_champ = simulate_series(get_higher_seed(east_sf_1, east_sf_2), get_lower_seed(east_sf_1, east_sf_2), rng).winner;
    let west_champ = simulate_series(get_higher_seed(west_sf_1, west_sf_2), get_lower_seed(west_sf_1, west_sf_2), rng).winner;

    // NBA Finals
    let finals_home = get_higher_seed(east_champ, west_champ);
    let finals_away = get_lower_seed(east_champ, west_champ);

    let champion = simulate_series(finals_home, finals_away, rng).winner;

    PostseasonResult {
        champion,
    }
}

// Helper functions to determine home court advantage (lower seed number = higher seed)
fn get_higher_seed(t1: TeamId, t2: TeamId) -> TeamId {
    let team1 = crate::core::teams::get_team(t1);
    let team2 = crate::core::teams::get_team(t2);
    
    if team1.seed < team2.seed {
        t1
    } else if team2.seed < team1.seed {
        t2
    } else {
        // Tie break by Net Rating
        let t1_net = team1.ortg - team1.drtg;
        let t2_net = team2.ortg - team2.drtg;
        if t1_net >= t2_net { t1 } else { t2 }
    }
}

fn get_lower_seed(t1: TeamId, t2: TeamId) -> TeamId {
    let higher = get_higher_seed(t1, t2);
    if higher == t1 { t2 } else { t1 }
}
