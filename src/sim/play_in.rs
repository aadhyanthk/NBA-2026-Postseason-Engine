use crate::core::types::TeamId;
use crate::core::rng::NbaRng;
use crate::sim::game::simulate_game;

/// Returns a tuple of (Seed 7, Seed 8) after the play-in tournament
pub fn simulate_play_in(
    seed7: TeamId,
    seed8: TeamId,
    seed9: TeamId,
    seed10: TeamId,
    rng: &mut NbaRng,
) -> (TeamId, TeamId) {
    // Game 1: 7 vs 8 (7 is home)
    let g1_result = simulate_game(seed7, seed8, rng);
    let g1_winner = g1_result.winner;
    let g1_loser = if g1_winner == seed7 { seed8 } else { seed7 };

    // Game 2: 9 vs 10 (9 is home)
    let g2_result = simulate_game(seed9, seed10, rng);
    let g2_winner = g2_result.winner;

    // Game 3: Loser of Game 1 vs Winner of Game 2 (Loser of Game 1 is home)
    let g3_result = simulate_game(g1_loser, g2_winner, rng);
    let final_seed8 = g3_result.winner;

    // The 7th seed is the winner of Game 1
    // The 8th seed is the winner of Game 3
    (g1_winner, final_seed8)
}
