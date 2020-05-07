extern crate rand;

pub mod glowworm;
pub mod swarm;

use swarm::Swarm;
use rand::SeedableRng;
use rand::rngs::StdRng;


#[derive(Debug)]
pub struct GSO {
    pub swarm: Swarm,
    pub rng: StdRng,
}


impl GSO {
    pub fn new(positions: &Vec<Vec<f64>>, seed: u64) -> GSO {
        let mut gso = GSO {
            swarm: Swarm::new(),
            rng: SeedableRng::seed_from_u64(seed),
        };
        gso.swarm.add_glowworms(positions);
        gso
    }

    pub fn run(&mut self, steps: u32) {
        for step in 1..steps+1 {
            println!("Step {}", step);
            self.swarm.update_luciferin();
            self.swarm.movement_phase(&mut self.rng);
        }
    }
}
