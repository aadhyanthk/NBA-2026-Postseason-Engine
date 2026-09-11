use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64Mcg;

pub struct NbaRng {
    rng: Pcg64Mcg,
}

impl NbaRng {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: Pcg64Mcg::seed_from_u64(seed),
        }
    }

    /// SplitMix64 avalanche function to thoroughly mix seed components
    fn splitmix64(mut x: u64) -> u64 {
        x = x.wrapping_add(0x9e3779b97f4a7c15);
        x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
        x ^ (x >> 31)
    }

    /// Hierarchical seed generation for deterministic parallel execution.
    pub fn from_seed_and_ids(global_seed: u64, sim_id: u64, game_id: u64) -> Self {
        // We use splitmix64 on each id layer and XOR them to combine
        let mut combined = Self::splitmix64(global_seed);
        combined ^= Self::splitmix64(sim_id);
        combined ^= Self::splitmix64(game_id);
        
        // One final avalanche ensures dense entropy distribution for the PCG seeder
        Self::new(Self::splitmix64(combined))
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
