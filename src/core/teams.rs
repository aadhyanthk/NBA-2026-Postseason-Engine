use super::types::{Conference, TeamId};

#[derive(Debug, Clone)]
pub struct Team {
    pub id: TeamId,
    pub name: &'static str,
    pub conf: Conference,
    pub seed: u8, // 1-10 for playoff/play-in teams, 0 for eliminated
    pub ortg: f32, // Offensive Rating
    pub drtg: f32, // Defensive Rating
    pub pace: f32, // Pace (Possessions per 48 min)
}

// 2023-2024 Season Stats (used as a proxy for the 2026 simulation baseline)
// We assign seeds 1-10 per conference for the simulation tournament.
pub const TEAMS: [Team; 30] = [
    // Eastern Conference
    Team { id: TeamId(0), name: "Boston Celtics", conf: Conference::East, seed: 1, ortg: 123.2, drtg: 111.6, pace: 97.2 },
    Team { id: TeamId(1), name: "New York Knicks", conf: Conference::East, seed: 2, ortg: 118.2, drtg: 112.4, pace: 95.8 },
    Team { id: TeamId(2), name: "Milwaukee Bucks", conf: Conference::East, seed: 3, ortg: 119.5, drtg: 115.8, pace: 100.5 },
    Team { id: TeamId(3), name: "Cleveland Cavaliers", conf: Conference::East, seed: 4, ortg: 115.2, drtg: 112.7, pace: 97.4 },
    Team { id: TeamId(4), name: "Orlando Magic", conf: Conference::East, seed: 5, ortg: 113.4, drtg: 111.3, pace: 97.3 },
    Team { id: TeamId(5), name: "Indiana Pacers", conf: Conference::East, seed: 6, ortg: 121.0, drtg: 118.0, pace: 102.2 },
    Team { id: TeamId(6), name: "Philadelphia 76ers", conf: Conference::East, seed: 7, ortg: 116.6, drtg: 113.8, pace: 98.0 },
    Team { id: TeamId(7), name: "Miami Heat", conf: Conference::East, seed: 8, ortg: 113.9, drtg: 112.2, pace: 96.9 },
    Team { id: TeamId(8), name: "Chicago Bulls", conf: Conference::East, seed: 9, ortg: 114.6, drtg: 116.3, pace: 96.9 },
    Team { id: TeamId(9), name: "Atlanta Hawks", conf: Conference::East, seed: 10, ortg: 117.2, drtg: 119.4, pace: 100.8 },
    Team { id: TeamId(10), name: "Brooklyn Nets", conf: Conference::East, seed: 0, ortg: 113.2, drtg: 116.1, pace: 97.7 },
    Team { id: TeamId(11), name: "Toronto Raptors", conf: Conference::East, seed: 0, ortg: 112.3, drtg: 118.8, pace: 99.3 },
    Team { id: TeamId(12), name: "Charlotte Hornets", conf: Conference::East, seed: 0, ortg: 109.3, drtg: 119.8, pace: 97.7 },
    Team { id: TeamId(13), name: "Washington Wizards", conf: Conference::East, seed: 0, ortg: 110.2, drtg: 119.6, pace: 102.7 },
    Team { id: TeamId(14), name: "Detroit Pistons", conf: Conference::East, seed: 0, ortg: 109.7, drtg: 119.0, pace: 99.1 },

    // Western Conference
    Team { id: TeamId(15), name: "Oklahoma City Thunder", conf: Conference::West, seed: 1, ortg: 119.5, drtg: 112.1, pace: 100.2 },
    Team { id: TeamId(16), name: "Denver Nuggets", conf: Conference::West, seed: 2, ortg: 118.5, drtg: 113.0, pace: 96.8 },
    Team { id: TeamId(17), name: "Minnesota Timberwolves", conf: Conference::West, seed: 3, ortg: 115.6, drtg: 109.0, pace: 97.8 },
    Team { id: TeamId(18), name: "LA Clippers", conf: Conference::West, seed: 4, ortg: 118.8, drtg: 115.4, pace: 97.8 },
    Team { id: TeamId(19), name: "Dallas Mavericks", conf: Conference::West, seed: 5, ortg: 117.6, drtg: 115.3, pace: 100.4 },
    Team { id: TeamId(20), name: "Phoenix Suns", conf: Conference::West, seed: 6, ortg: 117.6, drtg: 114.6, pace: 98.4 },
    Team { id: TeamId(21), name: "New Orleans Pelicans", conf: Conference::West, seed: 7, ortg: 117.4, drtg: 112.9, pace: 98.7 },
    Team { id: TeamId(22), name: "Los Angeles Lakers", conf: Conference::West, seed: 8, ortg: 115.9, drtg: 115.3, pace: 101.4 },
    Team { id: TeamId(23), name: "Sacramento Kings", conf: Conference::West, seed: 9, ortg: 116.9, drtg: 115.2, pace: 99.8 },
    Team { id: TeamId(24), name: "Golden State Warriors", conf: Conference::West, seed: 10, ortg: 117.8, drtg: 115.2, pace: 99.9 },
    Team { id: TeamId(25), name: "Houston Rockets", conf: Conference::West, seed: 0, ortg: 114.5, drtg: 113.4, pace: 99.1 },
    Team { id: TeamId(26), name: "Utah Jazz", conf: Conference::West, seed: 0, ortg: 115.5, drtg: 120.4, pace: 100.1 },
    Team { id: TeamId(27), name: "Memphis Grizzlies", conf: Conference::West, seed: 0, ortg: 107.2, drtg: 114.3, pace: 98.4 },
    Team { id: TeamId(28), name: "San Antonio Spurs", conf: Conference::West, seed: 0, ortg: 110.0, drtg: 116.4, pace: 101.5 },
    Team { id: TeamId(29), name: "Portland Trail Blazers", conf: Conference::West, seed: 0, ortg: 108.3, drtg: 119.6, pace: 97.7 },
];

pub fn get_team(id: TeamId) -> &'static Team {
    &TEAMS[id.0 as usize]
}
