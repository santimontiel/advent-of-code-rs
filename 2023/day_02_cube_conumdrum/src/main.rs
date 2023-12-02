use std::{fs, time::Instant};
use std::collections::HashMap;
use std::collections::HashSet;


fn part_1() {
    println!("Solving part 1... :)");
    let now: Instant = Instant::now();
    let input: String = fs::read_to_string("input.txt").unwrap();
    
    let cubes: HashMap<&str, u32> = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14)
    ]);

    let mut impossible_ids: HashSet<u32> = HashSet::new();
    let mut total_counter: u32 = 0;

    for line in input.lines() {

        let mg: Vec<_> = line.split(":").collect();
        let game_id: Vec<_> = mg[0].split(" ").collect();
        let id: u32 = game_id[1].parse().unwrap();
        total_counter += id;

        let rounds: Vec<_> = mg[1].trim().split("; ").collect();
        for round in rounds {
            let items: Vec<_> = round.split(", ").collect();
            
            for item in items {
                let qc: Vec<_> = item.split(" ").collect();
                let quantity: u32 = qc[0].parse().unwrap();
                let color: &str = qc[1];
                let max_quantity: u32 = cubes.get(color).unwrap().clone();
        
                if max_quantity < quantity {
                    impossible_ids.insert(id);
                }
            }
        } 
    }
    
    let result: u32 = total_counter - impossible_ids.iter().cloned().sum::<u32>();

    println!("{result}");
    let elapsed = (now.elapsed().as_nanos() as f32) / 1000.0;
    println!("Part 1 took {elapsed} us.")
}


fn part_2() {
    println!("Solving part 2... :)");
    let now: Instant = Instant::now();
    let input: String = fs::read_to_string("input.txt").unwrap();

    let mut result: u32 = 0;
    for line in input.lines() {

        let mut cubes: HashMap<&str, u32> = HashMap::from([
            ("red", 0),
            ("green", 0),
            ("blue", 0)
        ]);

        let mg: Vec<_> = line.split(":").collect();

        let rounds: Vec<_> = mg[1].trim().split("; ").collect();
        for round in rounds {
            let items: Vec<_> = round.split(", ").collect();
            
            for item in items {
                let qc: Vec<_> = item.split(" ").collect();
                let quantity: u32 = qc[0].parse().unwrap();
                let color: &str = qc[1];
                let max_quantity: u32 = cubes.get(color).unwrap().clone();
        
                if max_quantity < quantity {
                    if let Some(entry) = cubes.get_mut(color) {
                        *entry = quantity;
                    }
                }
            }
        } 
    
        result += cubes.get("red").unwrap().clone()
            * cubes.get("green").unwrap().clone()
            * cubes.get("blue").unwrap().clone() 
    }    

    println!("{result}");
    let elapsed = (now.elapsed().as_nanos() as f32) / 1000.0;
    println!("Part 2 took {elapsed} us.")
}


fn main() {
    println!("Hello, world!");
    part_1();
    part_2();
}
