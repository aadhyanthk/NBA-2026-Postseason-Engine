use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TeamId(pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Conference {
    East,
    West,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GameResult {
    pub home_team: TeamId,
    pub away_team: TeamId,
    pub home_score: u16,
    pub away_score: u16,
    pub winner: TeamId,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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

#[derive(Clone, Default, Serialize, Deserialize)]
#[repr(align(64))] // Force 64-byte cache-line alignment to prevent false sharing
pub struct SimAccumulator {
    pub play_in: [u32; 30],
    pub playoffs: [u32; 30],
    pub conf_finals: [u32; 30],
    pub finals: [u32; 30],
    pub championships: [u32; 30],
    pub total_games: u64,
}

impl SimAccumulator {
    pub fn merge(&mut self, other: &Self) {
        for i in 0..30 {
            self.play_in[i] += other.play_in[i];
            self.playoffs[i] += other.playoffs[i];
            self.conf_finals[i] += other.conf_finals[i];
            self.finals[i] += other.finals[i];
            self.championships[i] += other.championships[i];
        }
        self.total_games += other.total_games;
    }
}
