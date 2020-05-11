use super::{Biome, Block, River};
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Planet {
    pub rng_seed: u64,
    pub perlin_seed: u64,
    pub remaining_settlers: i32,
    pub migrant_counter: i32,
    pub water_divisor: i32,
    pub plains_divisor: i32,
    pub starting_settlers: i32,
    pub strict_beamdown: bool,

    pub water_height: u8,
    pub plains_height: u8,
    pub hills_height: u8,

    pub landblocks: Vec<Block>,
    pub biomes: Vec<Biome>,
    pub rivers: Vec<River>,
}

impl Planet {
    pub fn new() -> Self {
        Planet {
            rng_seed: 0,
            perlin_seed: 0,
            remaining_settlers: 0,
            migrant_counter: 0,
            water_divisor: 0,
            plains_divisor: 0,
            starting_settlers: 0,
            strict_beamdown: false,
            water_height: 0,
            plains_height: 0,
            hills_height: 0,
            landblocks: Vec::new(),
            biomes: Vec::new(),
            rivers: Vec::new(),
        }
    }
}

pub fn planet_idx(x: i32, y: i32) -> usize {
    (super::WORLD_WIDTH as usize * y as usize) + x as usize
}