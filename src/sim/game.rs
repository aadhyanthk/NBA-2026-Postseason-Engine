use crate::core::types::{GameResult, TeamId};
use crate::core::teams::get_team;
use crate::core::rng::NbaRng;

const LEAGUE_AVG_PACE: f32 = 99.0;
const LEAGUE_AVG_EFF: f32 = 115.0;
const HCA_BONUS: f32 = 3.2;

pub fn simulate_game(home: TeamId, away: TeamId, rng: &mut NbaRng) -> GameResult {
    let t_home = get_team(home);
    let t_away = get_team(away);

    // Calculate game pace based on team paces
    let pace = (t_home.pace * t_away.pace) / LEAGUE_AVG_PACE;

    // Calculate expected efficiencies
    let home_eff = t_home.ortg + t_away.drtg - LEAGUE_AVG_EFF + HCA_BONUS;
    let away_eff = t_away.ortg + t_home.drtg - LEAGUE_AVG_EFF;

    let home_ppp = home_eff / 100.0;
    let away_ppp = away_eff / 100.0;

    let mut home_score = 0;
    let mut away_score = 0;

    let total_possessions = pace.round() as u32;

    for _ in 0..total_possessions {
        home_score += simulate_possession(home_ppp, t_home.tov_pct, t_home.oreb_pct, t_home.three_point_rate, rng);
        away_score += simulate_possession(away_ppp, t_away.tov_pct, t_away.oreb_pct, t_away.three_point_rate, rng);
    }

    // Overtime resolution
    while home_score == away_score {
        for _ in 0..10 {
            home_score += simulate_possession(home_ppp, t_home.tov_pct, t_home.oreb_pct, t_home.three_point_rate, rng);
            away_score += simulate_possession(away_ppp, t_away.tov_pct, t_away.oreb_pct, t_away.three_point_rate, rng);
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

/// A simplified possession Markov model with Four Factors and dynamic variance.
#[inline]
fn simulate_possession(expected_ppp: f32, tov_pct: f32, oreb_pct: f32, three_point_rate: f32, rng: &mut NbaRng) -> u16 {
    let mut points = 0;

    for _ in 0..3 { // Cap offensive rebounds at 3
        if rng.gen_range(0.0, 1.0) < tov_pct {
            return points; 
        }

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

        let draw = rng.gen_range(0.0, 1.0);
        let mut scored = false;

        if draw < p3 {
            points += 3;
            scored = true;
        } else if draw < p3 + p2 {
            points += 2;
            scored = true;
        } else if draw < p3 + p2 + p1 {
            points += 1;
            scored = true;
        }

        if scored {
            break;
        } else {
            // Missed shot. Do we offensive rebound?
            if rng.gen_range(0.0, 1.0) < oreb_pct {
                continue;
            } else {
                break;
            }
        }
    }

    points
}
