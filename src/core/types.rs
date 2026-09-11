#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TeamId(pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Conference {
    East,
    West,
}

#[derive(Debug, Clone, Copy)]
pub struct GameResult {
    pub home_team: TeamId,
    pub away_team: TeamId,
    pub home_score: u16,
    pub away_score: u16,
    pub winner: TeamId,
}

#[derive(Debug, Clone, Copy)]
pub struct SeriesResult {
    pub team_a: TeamId, // Higher seed usually
    pub team_b: TeamId,
    pub team_a_wins: u8,
    pub team_b_wins: u8,
    pub winner: TeamId,
}

impl SeriesResult {
    pub fn new(team_a: TeamId, team_b: TeamId) -> Self {
        Self {
            team_a,
            team_b,
            team_a_wins: 0,
            team_b_wins: 0,
            winner: TeamId(255), // placeholder
        }
    }
}
