use gso::GSO;
use std::env;
use std::fs;


fn main() {
    // Parse command line
    let args: Vec<String> = env::args().collect();
    match args.len() {
        3 => {
            let swarm_filename = &args[1];
            let num_steps = &args[2];
            // parse the number
            let steps: u32 = match num_steps.parse() {
                Ok(n) => {
                    n
                },
                Err(_) => {
                    eprintln!("Error: second argument must be a number");
                    return;
                },
            };

            simulate(swarm_filename, steps);
        }
        _ => {
            println!("Wrong command line. Usage: {} swarm_filename steps", args[0]);
        }
    }
}

fn simulate(swarm_filename: &String, steps: u32) {
    let seed:u64 = 324324324;

    // Parse swarm filename content
    let contents = fs::read_to_string(swarm_filename)
        .expect("Error reading the input file");
    let mut positions: Vec<Vec<f64>> = Vec::new();
    for s in contents.lines() {
        let start_offset = s.find("(").unwrap_or(s.len());
        let end_offset = s.find(")").unwrap_or(s.len());
        if start_offset < end_offset {
            let vector_raw: String = String::from(s).drain(start_offset+1..end_offset).collect();
            let vector: Vec<&str> = vector_raw.split(",").collect();
            let mut position: Vec<f64> = Vec::new();
            for pos in vector.iter() {         
                position.push(pos.trim().parse::<f64>().unwrap());
            }
            positions.push(position);
        }
    }

    let mut gso = GSO::new(&positions, seed);

    // Print starting landscape positions
    for glowworm in gso.swarm.glowworms.iter() {
        println!("{} {:?}", glowworm.id, glowworm.landscape_position);
    }

    gso.run(steps);

    // Print final landscape positions
    for glowworm in gso.swarm.glowworms.iter() {
        println!("{} {:?}", glowworm.id, glowworm.landscape_position);
    }
}