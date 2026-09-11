use super::types::{Conference, TeamId};

#[derive(Debug, Clone)]
pub struct Team {
    pub id: TeamId,
    pub name: &'static str,
    pub conf: Conference,
    pub seed: u8, // 1-10 for playoff/play-in teams, 0 for eliminated
}

pub struct LeagueStatsSoA {
    pub ortg: [f32; 30],
    pub drtg: [f32; 30],
    pub pace: [f32; 30],
    pub tov_pct: [f32; 30],
    pub oreb_pct: [f32; 30],
    pub three_point_rate: [f32; 30],
}

// 2025-2026 NBA Season Stats & Official Postseason Standings
pub const TEAMS: [Team; 30] = [
    Team { id: TeamId(0), name: "Boston Celtics", conf: Conference::East, seed: 2 },
    Team { id: TeamId(1), name: "New York Knicks", conf: Conference::East, seed: 3 },
    Team { id: TeamId(2), name: "Milwaukee Bucks", conf: Conference::East, seed: 0 },
    Team { id: TeamId(3), name: "Cleveland Cavaliers", conf: Conference::East, seed: 4 },
    Team { id: TeamId(4), name: "Orlando Magic", conf: Conference::East, seed: 8 },
    Team { id: TeamId(5), name: "Indiana Pacers", conf: Conference::East, seed: 0 },
    Team { id: TeamId(6), name: "Philadelphia 76ers", conf: Conference::East, seed: 7 },
    Team { id: TeamId(7), name: "Miami Heat", conf: Conference::East, seed: 10 },
    Team { id: TeamId(8), name: "Chicago Bulls", conf: Conference::East, seed: 0 },
    Team { id: TeamId(9), name: "Atlanta Hawks", conf: Conference::East, seed: 6 },
    Team { id: TeamId(10), name: "Brooklyn Nets", conf: Conference::East, seed: 0 },
    Team { id: TeamId(11), name: "Toronto Raptors", conf: Conference::East, seed: 5 },
    Team { id: TeamId(12), name: "Charlotte Hornets", conf: Conference::East, seed: 9 },
    Team { id: TeamId(13), name: "Washington Wizards", conf: Conference::East, seed: 0 },
    Team { id: TeamId(14), name: "Detroit Pistons", conf: Conference::East, seed: 1 },

    Team { id: TeamId(15), name: "Oklahoma City Thunder", conf: Conference::West, seed: 1 },
    Team { id: TeamId(16), name: "Denver Nuggets", conf: Conference::West, seed: 3 },
    Team { id: TeamId(17), name: "Minnesota Timberwolves", conf: Conference::West, seed: 6 },
    Team { id: TeamId(18), name: "LA Clippers", conf: Conference::West, seed: 9 },
    Team { id: TeamId(19), name: "Dallas Mavericks", conf: Conference::West, seed: 5 },
    Team { id: TeamId(20), name: "Phoenix Suns", conf: Conference::West, seed: 7 },
    Team { id: TeamId(21), name: "New Orleans Pelicans", conf: Conference::West, seed: 0 },
    Team { id: TeamId(22), name: "Los Angeles Lakers", conf: Conference::West, seed: 4 },
    Team { id: TeamId(23), name: "Sacramento Kings", conf: Conference::West, seed: 0 },
    Team { id: TeamId(24), name: "Golden State Warriors", conf: Conference::West, seed: 8 },
    Team { id: TeamId(25), name: "Houston Rockets", conf: Conference::West, seed: 0 },
    Team { id: TeamId(26), name: "Utah Jazz", conf: Conference::West, seed: 0 },
    Team { id: TeamId(27), name: "Memphis Grizzlies", conf: Conference::West, seed: 0 },
    Team { id: TeamId(28), name: "San Antonio Spurs", conf: Conference::West, seed: 2 },
    Team { id: TeamId(29), name: "Portland Trail Blazers", conf: Conference::West, seed: 10 },
];

pub const LEAGUE_STATS_SOA: LeagueStatsSoA = LeagueStatsSoA {
    ortg: [
        121.5, 119.8, 116.5, 120.4, 113.8, 117.0, 116.0, 113.5, 112.8, 117.8, 111.4, 116.2, 114.2, 109.8, 118.8,
        121.8, 119.5, 116.4, 116.8, 118.1, 117.2, 112.5, 118.6, 115.8, 117.5, 115.2, 111.0, 113.0, 120.2, 113.6
    ],
    drtg: [
        111.8, 111.2, 118.0, 112.5, 111.4, 118.2, 114.9, 114.1, 117.5, 116.4, 118.2, 113.8, 116.8, 120.5, 109.5,
        108.2, 112.8, 110.8, 115.4, 114.0, 114.8, 116.2, 113.2, 116.5, 115.0, 114.6, 119.8, 115.8, 110.4, 116.2
    ],
    pace: [
        98.2, 97.5, 99.8, 98.8, 97.0, 101.5, 97.9, 96.8, 97.4, 101.2, 98.0, 99.1, 98.5, 102.1, 99.4,
        100.5, 97.6, 98.0, 97.5, 99.8, 98.6, 98.5, 101.0, 100.1, 100.2, 99.2, 99.6, 99.0, 101.8, 98.4
    ],
    tov_pct: [
        0.115, 0.120, 0.130, 0.135, 0.145, 0.125, 0.118, 0.128, 0.122, 0.135, 0.132, 0.140, 0.138, 0.145, 0.138,
        0.115, 0.125, 0.142, 0.130, 0.118, 0.135, 0.132, 0.138, 0.125, 0.140, 0.128, 0.148, 0.136, 0.140, 0.150
    ],
    oreb_pct: [
        0.245, 0.295, 0.240, 0.235, 0.265, 0.250, 0.255, 0.230, 0.260, 0.275, 0.248, 0.270, 0.235, 0.220, 0.285,
        0.230, 0.265, 0.245, 0.255, 0.250, 0.260, 0.250, 0.225, 0.245, 0.275, 0.270, 0.280, 0.255, 0.240, 0.265
    ],
    three_point_rate: [
        0.485, 0.410, 0.395, 0.420, 0.355, 0.380, 0.370, 0.390, 0.360, 0.405, 0.415, 0.375, 0.385, 0.390, 0.365,
        0.400, 0.355, 0.380, 0.395, 0.445, 0.385, 0.380, 0.365, 0.425, 0.450, 0.370, 0.410, 0.395, 0.410, 0.350
    ],
};

pub fn get_team(id: TeamId) -> &'static Team {
    &TEAMS[id.0 as usize]
}
