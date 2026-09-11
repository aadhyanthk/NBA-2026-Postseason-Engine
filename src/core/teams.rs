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
    pub tov_pct: f32, // Turnover percentage (e.g. 0.13)
    pub oreb_pct: f32, // Offensive Rebound percentage (e.g. 0.25)
    pub three_point_rate: f32, // 3-Point attempt rate (e.g. 0.40)
}

// 2025-2026 NBA Season Stats & Official Postseason Standings
// Seeds 1-10 per conference participate in the Play-In / Playoffs; Seed 0 are eliminated.
pub const TEAMS: [Team; 30] = [
    // Eastern Conference
    Team { id: TeamId(0), name: "Boston Celtics", conf: Conference::East, seed: 2, ortg: 121.5, drtg: 111.8, pace: 98.2, tov_pct: 0.115, oreb_pct: 0.245, three_point_rate: 0.485 },
    Team { id: TeamId(1), name: "New York Knicks", conf: Conference::East, seed: 3, ortg: 119.8, drtg: 111.2, pace: 97.5, tov_pct: 0.120, oreb_pct: 0.295, three_point_rate: 0.410 },
    Team { id: TeamId(2), name: "Milwaukee Bucks", conf: Conference::East, seed: 0, ortg: 116.5, drtg: 118.0, pace: 99.8, tov_pct: 0.130, oreb_pct: 0.240, three_point_rate: 0.395 },
    Team { id: TeamId(3), name: "Cleveland Cavaliers", conf: Conference::East, seed: 4, ortg: 120.4, drtg: 112.5, pace: 98.8, tov_pct: 0.135, oreb_pct: 0.235, three_point_rate: 0.420 },
    Team { id: TeamId(4), name: "Orlando Magic", conf: Conference::East, seed: 8, ortg: 113.8, drtg: 111.4, pace: 97.0, tov_pct: 0.145, oreb_pct: 0.265, three_point_rate: 0.355 },
    Team { id: TeamId(5), name: "Indiana Pacers", conf: Conference::East, seed: 0, ortg: 117.0, drtg: 118.2, pace: 101.5, tov_pct: 0.125, oreb_pct: 0.250, three_point_rate: 0.380 },
    Team { id: TeamId(6), name: "Philadelphia 76ers", conf: Conference::East, seed: 7, ortg: 116.0, drtg: 114.9, pace: 97.9, tov_pct: 0.118, oreb_pct: 0.255, three_point_rate: 0.370 },
    Team { id: TeamId(7), name: "Miami Heat", conf: Conference::East, seed: 10, ortg: 113.5, drtg: 114.1, pace: 96.8, tov_pct: 0.128, oreb_pct: 0.230, three_point_rate: 0.390 },
    Team { id: TeamId(8), name: "Chicago Bulls", conf: Conference::East, seed: 0, ortg: 112.8, drtg: 117.5, pace: 97.4, tov_pct: 0.122, oreb_pct: 0.260, three_point_rate: 0.360 },
    Team { id: TeamId(9), name: "Atlanta Hawks", conf: Conference::East, seed: 6, ortg: 117.8, drtg: 116.4, pace: 101.2, tov_pct: 0.135, oreb_pct: 0.275, three_point_rate: 0.405 },
    Team { id: TeamId(10), name: "Brooklyn Nets", conf: Conference::East, seed: 0, ortg: 111.4, drtg: 118.2, pace: 98.0, tov_pct: 0.132, oreb_pct: 0.248, three_point_rate: 0.415 },
    Team { id: TeamId(11), name: "Toronto Raptors", conf: Conference::East, seed: 5, ortg: 116.2, drtg: 113.8, pace: 99.1, tov_pct: 0.140, oreb_pct: 0.270, three_point_rate: 0.375 },
    Team { id: TeamId(12), name: "Charlotte Hornets", conf: Conference::East, seed: 9, ortg: 114.2, drtg: 116.8, pace: 98.5, tov_pct: 0.138, oreb_pct: 0.235, three_point_rate: 0.385 },
    Team { id: TeamId(13), name: "Washington Wizards", conf: Conference::East, seed: 0, ortg: 109.8, drtg: 120.5, pace: 102.1, tov_pct: 0.145, oreb_pct: 0.220, three_point_rate: 0.390 },
    Team { id: TeamId(14), name: "Detroit Pistons", conf: Conference::East, seed: 1, ortg: 118.8, drtg: 109.5, pace: 99.4, tov_pct: 0.138, oreb_pct: 0.285, three_point_rate: 0.365 },

    // Western Conference
    Team { id: TeamId(15), name: "Oklahoma City Thunder", conf: Conference::West, seed: 1, ortg: 121.8, drtg: 108.2, pace: 100.5, tov_pct: 0.115, oreb_pct: 0.230, three_point_rate: 0.400 },
    Team { id: TeamId(16), name: "Denver Nuggets", conf: Conference::West, seed: 3, ortg: 119.5, drtg: 112.8, pace: 97.6, tov_pct: 0.125, oreb_pct: 0.265, three_point_rate: 0.355 },
    Team { id: TeamId(17), name: "Minnesota Timberwolves", conf: Conference::West, seed: 6, ortg: 116.4, drtg: 110.8, pace: 98.0, tov_pct: 0.142, oreb_pct: 0.245, three_point_rate: 0.380 },
    Team { id: TeamId(18), name: "LA Clippers", conf: Conference::West, seed: 9, ortg: 116.8, drtg: 115.4, pace: 97.5, tov_pct: 0.130, oreb_pct: 0.255, three_point_rate: 0.395 },
    Team { id: TeamId(19), name: "Dallas Mavericks", conf: Conference::West, seed: 5, ortg: 118.1, drtg: 114.0, pace: 99.8, tov_pct: 0.118, oreb_pct: 0.250, three_point_rate: 0.445 },
    Team { id: TeamId(20), name: "Phoenix Suns", conf: Conference::West, seed: 7, ortg: 117.2, drtg: 114.8, pace: 98.6, tov_pct: 0.135, oreb_pct: 0.260, three_point_rate: 0.385 },
    Team { id: TeamId(21), name: "New Orleans Pelicans", conf: Conference::West, seed: 0, ortg: 112.5, drtg: 116.2, pace: 98.5, tov_pct: 0.132, oreb_pct: 0.250, three_point_rate: 0.380 },
    Team { id: TeamId(22), name: "Los Angeles Lakers", conf: Conference::West, seed: 4, ortg: 118.6, drtg: 113.2, pace: 101.0, tov_pct: 0.138, oreb_pct: 0.225, three_point_rate: 0.365 },
    Team { id: TeamId(23), name: "Sacramento Kings", conf: Conference::West, seed: 0, ortg: 115.8, drtg: 116.5, pace: 100.1, tov_pct: 0.125, oreb_pct: 0.245, three_point_rate: 0.425 },
    Team { id: TeamId(24), name: "Golden State Warriors", conf: Conference::West, seed: 8, ortg: 117.5, drtg: 115.0, pace: 100.2, tov_pct: 0.140, oreb_pct: 0.275, three_point_rate: 0.450 },
    Team { id: TeamId(25), name: "Houston Rockets", conf: Conference::West, seed: 0, ortg: 115.2, drtg: 114.6, pace: 99.2, tov_pct: 0.128, oreb_pct: 0.270, three_point_rate: 0.370 },
    Team { id: TeamId(26), name: "Utah Jazz", conf: Conference::West, seed: 0, ortg: 111.0, drtg: 119.8, pace: 99.6, tov_pct: 0.148, oreb_pct: 0.280, three_point_rate: 0.410 },
    Team { id: TeamId(27), name: "Memphis Grizzlies", conf: Conference::West, seed: 0, ortg: 113.0, drtg: 115.8, pace: 99.0, tov_pct: 0.136, oreb_pct: 0.255, three_point_rate: 0.395 },
    Team { id: TeamId(28), name: "San Antonio Spurs", conf: Conference::West, seed: 2, ortg: 120.2, drtg: 110.4, pace: 101.8, tov_pct: 0.140, oreb_pct: 0.240, three_point_rate: 0.410 },
    Team { id: TeamId(29), name: "Portland Trail Blazers", conf: Conference::West, seed: 10, ortg: 113.6, drtg: 116.2, pace: 98.4, tov_pct: 0.150, oreb_pct: 0.265, three_point_rate: 0.350 },
];

pub fn get_team(id: TeamId) -> &'static Team {
    &TEAMS[id.0 as usize]
}
