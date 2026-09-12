use crate::core::types::{SeriesResult, TeamId};
use crate::core::seasons::SeasonData;
use crate::core::rng::NbaRng;
use crate::sim::game::simulate_game;

pub fn simulate_series(team_a: TeamId, team_b: TeamId, rng: &mut NbaRng, season: &SeasonData) -> SeriesResult {
    let mut wins_a = 0;
    let mut wins_b = 0;

    // 2-2-1-1-1 format. team_a is assumed to be the higher seed.
    let home_schedule = [
        team_a, team_a, team_b, team_b, team_a, team_b, team_a
    ];

    for &home_team in &home_schedule {
        let away_team = if home_team == team_a { team_b } else { team_a };
        
        let result = simulate_game(home_team, away_team, rng, season);
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

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]
        #[test]
        fn fuzz_series_invariants(seed in any::<u64>(), t1_id in 0u8..15, t2_id in 15u8..30) {
            let mut rng = NbaRng::from_seed_and_ids(seed, 0, 0);
            let team_a = TeamId(t1_id);
            let team_b = TeamId(t2_id);
            
            let season = &crate::core::seasons::SEASONS[10]; // use 2026 for tests
            let result = simulate_series(team_a, team_b, &mut rng, season);
            
            // Invariants of a Best-of-7 series:
            let total_games = result.team_a_wins + result.team_b_wins;
            
            // 1. Series must last between 4 and 7 games
            prop_assert!(total_games >= 4 && total_games <= 7);
            
            // 2. The winner must have EXACTLY 4 wins
            let winner_wins = if result.winner == team_a { result.team_a_wins } else { result.team_b_wins };
            prop_assert_eq!(winner_wins, 4);
            
            // 3. The loser must have strictly fewer than 4 wins
            let loser_wins = if result.winner == team_a { result.team_b_wins } else { result.team_a_wins };
            prop_assert!(loser_wins < 4);
        }
    }
}
