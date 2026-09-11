use crate::core::types::{Conference, TeamId};
use crate::core::rng::NbaRng;
use crate::core::teams::TEAMS;
use crate::sim::play_in::simulate_play_in;
use crate::sim::playoffs::simulate_series;

pub struct PostseasonResult {
    pub play_in_teams: [TeamId; 8],
    pub playoff_teams: [TeamId; 16],
    pub conf_finals_teams: [TeamId; 4],
    pub finals_teams: [TeamId; 2],
    pub champion: TeamId,
    pub total_games: u32,
}

pub fn simulate_postseason(rng: &mut NbaRng) -> PostseasonResult {
    let mut total_games = 0u32;

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

    let play_in_teams = [
        east_seeds[7], east_seeds[8], east_seeds[9], east_seeds[10],
        west_seeds[7], west_seeds[8], west_seeds[9], west_seeds[10],
    ];

    // Play-in Tournament (3 games each conference = 6 games)
    let (east_7, east_8) = simulate_play_in(east_seeds[7], east_seeds[8], east_seeds[9], east_seeds[10], rng);
    east_seeds[7] = east_7;
    east_seeds[8] = east_8;
    total_games += 3;

    let (west_7, west_8) = simulate_play_in(west_seeds[7], west_seeds[8], west_seeds[9], west_seeds[10], rng);
    west_seeds[7] = west_7;
    west_seeds[8] = west_8;
    total_games += 3;

    let playoff_teams = [
        east_seeds[1], east_seeds[2], east_seeds[3], east_seeds[4],
        east_seeds[5], east_seeds[6], east_seeds[7], east_seeds[8],
        west_seeds[1], west_seeds[2], west_seeds[3], west_seeds[4],
        west_seeds[5], west_seeds[6], west_seeds[7], west_seeds[8],
    ];

    // Playoffs Round 1 (First Round)
    let s_e1 = simulate_series(east_seeds[1], east_seeds[8], rng);
    total_games += (s_e1.team_a_wins + s_e1.team_b_wins) as u32;
    let east_r1_1 = s_e1.winner;

    let s_e4 = simulate_series(east_seeds[4], east_seeds[5], rng);
    total_games += (s_e4.team_a_wins + s_e4.team_b_wins) as u32;
    let east_r1_4 = s_e4.winner;

    let s_e3 = simulate_series(east_seeds[3], east_seeds[6], rng);
    total_games += (s_e3.team_a_wins + s_e3.team_b_wins) as u32;
    let east_r1_3 = s_e3.winner;

    let s_e2 = simulate_series(east_seeds[2], east_seeds[7], rng);
    total_games += (s_e2.team_a_wins + s_e2.team_b_wins) as u32;
    let east_r1_2 = s_e2.winner;

    let s_w1 = simulate_series(west_seeds[1], west_seeds[8], rng);
    total_games += (s_w1.team_a_wins + s_w1.team_b_wins) as u32;
    let west_r1_1 = s_w1.winner;

    let s_w4 = simulate_series(west_seeds[4], west_seeds[5], rng);
    total_games += (s_w4.team_a_wins + s_w4.team_b_wins) as u32;
    let west_r1_4 = s_w4.winner;

    let s_w3 = simulate_series(west_seeds[3], west_seeds[6], rng);
    total_games += (s_w3.team_a_wins + s_w3.team_b_wins) as u32;
    let west_r1_3 = s_w3.winner;

    let s_w2 = simulate_series(west_seeds[2], west_seeds[7], rng);
    total_games += (s_w2.team_a_wins + s_w2.team_b_wins) as u32;
    let west_r1_2 = s_w2.winner;

    // Playoffs Round 2 (Conference Semifinals)
    let s_esf1 = simulate_series(get_higher_seed(east_r1_1, east_r1_4), get_lower_seed(east_r1_1, east_r1_4), rng);
    total_games += (s_esf1.team_a_wins + s_esf1.team_b_wins) as u32;
    let east_sf_1 = s_esf1.winner;

    let s_esf2 = simulate_series(get_higher_seed(east_r1_2, east_r1_3), get_lower_seed(east_r1_2, east_r1_3), rng);
    total_games += (s_esf2.team_a_wins + s_esf2.team_b_wins) as u32;
    let east_sf_2 = s_esf2.winner;

    let s_wsf1 = simulate_series(get_higher_seed(west_r1_1, west_r1_4), get_lower_seed(west_r1_1, west_r1_4), rng);
    total_games += (s_wsf1.team_a_wins + s_wsf1.team_b_wins) as u32;
    let west_sf_1 = s_wsf1.winner;

    let s_wsf2 = simulate_series(get_higher_seed(west_r1_2, west_r1_3), get_lower_seed(west_r1_2, west_r1_3), rng);
    total_games += (s_wsf2.team_a_wins + s_wsf2.team_b_wins) as u32;
    let west_sf_2 = s_wsf2.winner;

    let conf_finals_teams = [east_sf_1, east_sf_2, west_sf_1, west_sf_2];

    // Playoffs Round 3 (Conference Finals)
    let s_ecf = simulate_series(get_higher_seed(east_sf_1, east_sf_2), get_lower_seed(east_sf_1, east_sf_2), rng);
    total_games += (s_ecf.team_a_wins + s_ecf.team_b_wins) as u32;
    let east_champ = s_ecf.winner;

    let s_wcf = simulate_series(get_higher_seed(west_sf_1, west_sf_2), get_lower_seed(west_sf_1, west_sf_2), rng);
    total_games += (s_wcf.team_a_wins + s_wcf.team_b_wins) as u32;
    let west_champ = s_wcf.winner;

    let finals_teams = [east_champ, west_champ];

    // NBA Finals
    let finals_home = get_higher_seed(east_champ, west_champ);
    let finals_away = get_lower_seed(east_champ, west_champ);

    let s_finals = simulate_series(finals_home, finals_away, rng);
    total_games += (s_finals.team_a_wins + s_finals.team_b_wins) as u32;
    let champion = s_finals.winner;

    PostseasonResult {
        play_in_teams,
        playoff_teams,
        conf_finals_teams,
        finals_teams,
        champion,
        total_games,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulate_postseason_invariants() {
        let mut rng = NbaRng::from_seed_and_ids(42, 1, 0);
        let result = simulate_postseason(&mut rng);

        // Valid total games: 6 play-in + 15 series * [4, 7] = [66, 111]
        assert!(result.total_games >= 66 && result.total_games <= 111);
        
        // Champion must be one of the 2 finals teams
        assert!(result.finals_teams.contains(&result.champion));

        // Exactly 8 playoff teams per conference
        assert_eq!(result.playoff_teams.len(), 16);
    }

    #[test]
    fn test_simulate_postseason_determinism() {
        let mut rng1 = NbaRng::from_seed_and_ids(999, 10, 0);
        let res1 = simulate_postseason(&mut rng1);

        let mut rng2 = NbaRng::from_seed_and_ids(999, 10, 0);
        let res2 = simulate_postseason(&mut rng2);

        assert_eq!(res1.champion, res2.champion);
        assert_eq!(res1.total_games, res2.total_games);
        assert_eq!(res1.finals_teams, res2.finals_teams);
    }
}

