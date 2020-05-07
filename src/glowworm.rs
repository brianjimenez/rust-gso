use std::f64::consts::PI;
use std::f64;


#[derive(Debug)]
pub struct Glowworm {
    pub id: u32,
    pub landscape_position: Vec<f64>,
    pub rho: f64,
    pub gamma: f64,
    pub beta: f64,
    pub luciferin: f64,
    pub vision_range: f64,
    pub max_vision_range: f64,
    pub max_neighbors: u32,
    pub neighbors: Vec<u32>,
    pub probabilities: Vec<f64>,
    pub scoring: f64,
    pub moved: bool,
    pub step: u32,
}


impl Glowworm {
    pub fn new(id: u32, x:f64, y:f64) -> Glowworm {
        Glowworm {
            id: id,
            landscape_position: [x, y].to_vec(),
            rho: 0.5,
            gamma: 0.4,
            beta: 0.08,
            luciferin: 5.0,
            vision_range: 0.2,
            max_vision_range: 5.0,
            max_neighbors: 5,
            neighbors: Vec::new(),
            probabilities: Vec::new(),
            scoring: 0.0,
            moved: false,
            step: 0,
        }
    }

    pub fn compute_luciferin(&mut self) {
        self.scoring = rastrigin(self.landscape_position[0], self.landscape_position[1]);
        self.luciferin = (1.0 - self.rho) * self.luciferin + self.gamma * self.scoring;
        self.step += 1;
    }

    pub fn distance(&mut self, other: &Glowworm) -> f64 {
        let x1 = self.landscape_position[0];
        let x2 = other.landscape_position[0];
        let y1 = self.landscape_position[1];
        let y2 = other.landscape_position[2];
        return ((x1-x2)*(x1-x2) + (y1-y2)*(y1-y2)).sqrt();
    }

    pub fn is_neighbor(&mut self, other: &Glowworm) -> bool {
        if self.id != other.id && self.luciferin < other.luciferin {
            return self.distance(other) < self.vision_range;
        }
        return false;
    }

    pub fn update_vision_range(&mut self) {
        self.vision_range = (self.max_vision_range).min(
            (0_f64).max(self.vision_range + self.beta * f64::from(self.max_neighbors as i32 - 
                        (self.neighbors.len() as i32))));
    }

    pub fn compute_probability_moving_toward_neighbor(&mut self, luciferins: &Vec<f64>) {
        self.probabilities = Vec::new();

        let mut total_sum: f64 = 0.0;
        let mut difference: f64;
        for neighbor_id in &self.neighbors {
            difference = luciferins[*neighbor_id as usize] - self.luciferin;
            self.probabilities.push(difference);
            total_sum += difference;
        }

        for i in 0..self.neighbors.len(){
            self.probabilities[i] /= total_sum;
        }
    }


    pub fn select_random_neighbor(&mut self, random_number:f64) -> u32 {
        if self.neighbors.len() == 0 {
            return self.id;
        }

        let mut sum_probabilities:f64 = 0.0;
        let mut i:usize = 0;
        while sum_probabilities < random_number {
            sum_probabilities += f64::from(self.probabilities[i]);
            i += 1;
        }
        return self.neighbors[i-1];
    }

    pub fn move_towards(&mut self, other_id:u32, new_position:&Vec<f64>) {
        self.moved = self.id != other_id;
        if self.id != other_id {
            let mut delta_x:Vec<f64> = vec![new_position[0] - self.landscape_position[0],
                                            new_position[1] - self.landscape_position[1]];
            let norm:f64 = (delta_x[0]*delta_x[0] + delta_x[1]*delta_x[1]).sqrt();
            let coef:f64 = 0.03 / norm;
            delta_x[0] *= coef;
            delta_x[1] *= coef;
            self.landscape_position[0] += delta_x[0];
            self.landscape_position[1] += delta_x[1];
        }
    }
}

pub fn distance(one: &Glowworm, two: &Glowworm) -> f64 {
    let x1 = one.landscape_position[0];
    let x2 = two.landscape_position[0];
    let y1 = one.landscape_position[1];
    let y2 = two.landscape_position[1];
    return ((x1-x2)*(x1-x2) + (y1-y2)*(y1-y2)).sqrt();
}


pub fn rastrigin(x: f64, y: f64) -> f64 {
    20.0 + (x*x - 10.0 * (2.0 * PI * x).cos() + y * y - 10.0 * (2.0 * PI * y).cos())
}
