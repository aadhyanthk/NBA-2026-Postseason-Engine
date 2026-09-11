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
        home_score += simulate_possession(home_ppp, rng);
        away_score += simulate_possession(away_ppp, rng);
    }

    // Overtime resolution
    while home_score == away_score {
        for _ in 0..10 {
            home_score += simulate_possession(home_ppp, rng);
            away_score += simulate_possession(away_ppp, rng);
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

/// A simplified possession Markov model.
/// Maps expected points per possession (PPP) to discrete outcomes (0, 1, 2, 3 points).
#[inline]
fn simulate_possession(expected_ppp: f32, rng: &mut NbaRng) -> u16 {
    let draw = rng.gen_range(0.0, 1.0);
    
    // Baseline probabilities that roughly sum to a PPP of 1.15
    // P(3) = 12%  => 0.36
    // P(2) = 35%  => 0.70
    // P(1) = 9%   => 0.09
    // Sum = 1.15
    let scaling = expected_ppp / 1.15;
    
    let p3 = 0.12 * scaling;
    let p2 = 0.35 * scaling;
    let p1 = 0.09 * scaling;

    if draw < p3 {
        3
    } else if draw < p3 + p2 {
        2
    } else if draw < p3 + p2 + p1 {
        1
    } else {
        0
    }
}
