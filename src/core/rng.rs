use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

pub struct NbaRng {
    rng: ChaCha8Rng,
}

impl NbaRng {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: ChaCha8Rng::seed_from_u64(seed),
        }
    }

    /// Hierarchical seed generation for deterministic parallel execution.
    pub fn from_seed_and_ids(global_seed: u64, sim_id: u64, game_id: u64) -> Self {
        // A simple hash function to combine the IDs (can be replaced with Philox/PCG later)
        let mut combined = global_seed;
        combined = combined.wrapping_add(sim_id).wrapping_mul(0x9E3779B97F4A7C15);
        combined = combined.wrapping_add(game_id).wrapping_mul(0x9E3779B97F4A7C15);
        Self::new(combined)
    }

    pub fn gen_range(&mut self, low: f32, high: f32) -> f32 {
        self.rng.gen_range(low..high)
    }

    pub fn gen_bool(&mut self, probability: f32) -> bool {
        self.rng.gen_bool(probability as f64)
    }
    
    pub fn gen_u16_range(&mut self, low: u16, high: u16) -> u16 {
        self.rng.gen_range(low..high)
    }
}
