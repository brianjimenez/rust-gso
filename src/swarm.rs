use super::glowworm::Glowworm;
use super::glowworm::distance;
use rand::Rng;


#[derive(Debug)]
pub struct Swarm {
	pub glowworms: Vec<Glowworm>,
}


impl Swarm {
    pub fn new() -> Swarm {
    	Swarm {
    		glowworms: Vec::new(),
    	}
    }

    pub fn add_glowworms(&mut self, positions: &Vec<Vec<f64>>) {
    	for i in 0..positions.len() {
    		let x = positions[i][0];
    		let y = positions[i][1];
    		self.glowworms.push(Glowworm::new(i as u32, x, y));
    	}
    }

    pub fn update_luciferin(&mut self) {
    	for glowworm in self.glowworms.iter_mut() {
    		glowworm.compute_luciferin();
    	}
    }

    pub fn movement_phase(&mut self, rng: &mut rand::prelude::StdRng) {
    	// Save original positions
    	let mut positions: Vec<Vec<f64>> = Vec::new();
    	for glowworm in self.glowworms.iter(){
    		positions.push(glowworm.landscape_position.clone());
    	}

    	// First search for each glowworm's neighbors
    	let mut neighbors: Vec<Vec<u32>> = Vec::new();
    	for i in 0..self.glowworms.len() {
    		let mut this_neighbors = Vec::new();
    		let g1 = &self.glowworms[i];
        	for j in 0..self.glowworms.len() {
        		if i != j {
        			let g2 = &self.glowworms[j];
        			if g1.luciferin < g2.luciferin {
        				let distance = distance(g1, g2);
            			if distance < g1.vision_range {
            				this_neighbors.push(g2.id);
            			}   
            		}
        		}
        	}
        	neighbors.push(this_neighbors);
    	}
    	
    	// Second compute probability moving towards the neighbor
    	let mut luciferins = Vec::new();
    	for glowworm in self.glowworms.iter_mut() {
    		luciferins.push(glowworm.luciferin);
    	}
    	for i in 0..self.glowworms.len() {
    		let glowworm = &mut self.glowworms[i];
    		glowworm.neighbors = neighbors[i].clone();
    		glowworm.compute_probability_moving_toward_neighbor(&luciferins);
    	}
    	
    	// Finally move to the selected position
        for i in 0..self.glowworms.len() {
        	let glowworm = &mut self.glowworms[i];
            let neighbor_id = glowworm.select_random_neighbor(rng.gen::<f64>());
            let position = &positions[neighbor_id as usize];
            glowworm.move_towards(neighbor_id, position);
            glowworm.update_vision_range();
        }
    }
}
