use crate::core::types::{GameResult, TeamId};
use crate::core::teams::LEAGUE_STATS_SOA;
use crate::core::rng::NbaRng;

const LEAGUE_AVG_PACE: f32 = 99.0;
const LEAGUE_AVG_EFF: f32 = 115.0;
const HCA_BONUS: f32 = 3.2;

pub fn simulate_game(home: TeamId, away: TeamId, rng: &mut NbaRng) -> GameResult {
    let h_idx = home.0 as usize;
    let a_idx = away.0 as usize;

    let h_pace = LEAGUE_STATS_SOA.pace[h_idx];
    let a_pace = LEAGUE_STATS_SOA.pace[a_idx];

    let h_ortg = LEAGUE_STATS_SOA.ortg[h_idx];
    let a_ortg = LEAGUE_STATS_SOA.ortg[a_idx];

    let h_drtg = LEAGUE_STATS_SOA.drtg[h_idx];
    let a_drtg = LEAGUE_STATS_SOA.drtg[a_idx];

    let h_tov = LEAGUE_STATS_SOA.tov_pct[h_idx];
    let a_tov = LEAGUE_STATS_SOA.tov_pct[a_idx];

    let h_oreb = LEAGUE_STATS_SOA.oreb_pct[h_idx];
    let a_oreb = LEAGUE_STATS_SOA.oreb_pct[a_idx];

    let h_3pr = LEAGUE_STATS_SOA.three_point_rate[h_idx];
    let a_3pr = LEAGUE_STATS_SOA.three_point_rate[a_idx];

    // Calculate game pace based on team paces
    let pace = (h_pace * a_pace) / LEAGUE_AVG_PACE;

    // Calculate expected efficiencies
    let home_eff = h_ortg + a_drtg - LEAGUE_AVG_EFF + HCA_BONUS;
    let away_eff = a_ortg + h_drtg - LEAGUE_AVG_EFF;

    let home_ppp = home_eff / 100.0;
    let away_ppp = away_eff / 100.0;

    let (h_p3, h_p2, h_p1) = calc_shot_probs(home_ppp, h_tov, h_3pr);
    let (a_p3, a_p2, a_p1) = calc_shot_probs(away_ppp, a_tov, a_3pr);

    let mut home_score = 0;
    let mut away_score = 0;

    let total_possessions = pace.round() as u32;

    for _ in 0..total_possessions {
        home_score += simulate_possession(h_tov, h_oreb, h_p3, h_p2, h_p1, rng);
        away_score += simulate_possession(a_tov, a_oreb, a_p3, a_p2, a_p1, rng);
    }

    // Overtime resolution
    while home_score == away_score {
        for _ in 0..10 {
            home_score += simulate_possession(h_tov, h_oreb, h_p3, h_p2, h_p1, rng);
            away_score += simulate_possession(a_tov, a_oreb, a_p3, a_p2, a_p1, rng);
        }
    }

    GameResult {
        home_team: home,
        away_team: away,
        home_score,
        away_score,
        winner: if home_score > away_score { home } else { away },
    }
}

#[inline]
fn calc_shot_probs(expected_ppp: f32, tov_pct: f32, three_point_rate: f32) -> (f32, f32, f32) {
    // Re-scale the expected_ppp to account for the fact that TOV% wastes possessions
    let non_tov_ppp = expected_ppp / (1.0 - tov_pct);
    let scaling = non_tov_ppp / 1.15;

    // Base probabilities
    let base_p3 = 0.12 * scaling;
    let base_p2 = 0.35 * scaling;
    let base_p1 = 0.09 * scaling;

    // Dynamic 3P variance (assume ~0.40 is league average)
    let shift_factor = three_point_rate / 0.40;
    let p3 = base_p3 * shift_factor;
    // Shift probability away from 2-pointers to compensate
    let p2 = base_p2 - (p3 - base_p3).max(0.0);
    let p1 = base_p1;

    (p3, p2, p1)
}

/// A simplified possession Markov model with Four Factors and dynamic variance.
#[inline]
fn simulate_possession(tov_pct: f32, oreb_pct: f32, p3: f32, p2: f32, p1: f32, rng: &mut NbaRng) -> u16 {
    let mut points = 0;

    let p2_threshold = p3 + p2;
    let p1_threshold = p3 + p2 + p1;

    for _ in 0..3 { // Cap offensive rebounds at 3
        if rng.gen_range(0.0, 1.0) < tov_pct {
            return points; 
        }

        let draw = rng.gen_range(0.0, 1.0);
        
        // Branchless arithmetic selection to avoid branch misprediction pipeline flushes
        let pts = ((draw < p3) as u16 * 3)
                + ((draw >= p3 && draw < p2_threshold) as u16 * 2)
                + ((draw >= p2_threshold && draw < p1_threshold) as u16 * 1);

        points += pts;

        if pts > 0 {
            break;
        } else {
            // Missed shot. Do we offensive rebound?
            if rng.gen_range(0.0, 1.0) >= oreb_pct {
                break;
            }
        }
    }

    points
}
