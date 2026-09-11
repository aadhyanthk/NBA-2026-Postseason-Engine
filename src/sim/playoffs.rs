use crate::core::types::{SeriesResult, TeamId};
use crate::core::rng::NbaRng;
use crate::sim::game::simulate_game;

pub fn simulate_series(team_a: TeamId, team_b: TeamId, rng: &mut NbaRng) -> SeriesResult {
    let mut wins_a = 0;
    let mut wins_b = 0;

    // 2-2-1-1-1 format. team_a is assumed to be the higher seed.
    let home_schedule = [
        team_a, team_a, team_b, team_b, team_a, team_b, team_a
    ];

    for &home_team in &home_schedule {
        let away_team = if home_team == team_a { team_b } else { team_a };
        
        let result = simulate_game(home_team, away_team, rng);
        if result.winner == team_a {
            wins_a += 1;
        } else {
            wins_b += 1;
        }

        if wins_a == 4 || wins_b == 4 {
            break;
        }
    }

    SeriesResult {
        team_a,
        team_b,
        team_a_wins: wins_a,
        team_b_wins: wins_b,
        winner: if wins_a == 4 { team_a } else { team_b },
    }
}
