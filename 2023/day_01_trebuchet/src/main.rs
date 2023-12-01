use std::{fs, time::Instant};
use std::collections::HashMap;


fn part_1() {
    println!("Solving part 1... :)");
    let now: Instant = Instant::now();
    let input: String = fs::read_to_string("input.txt").unwrap();

    let mut cnt = 0;
    for line in input.lines() {
        let mut chars: String = String::new();
        for char in line.chars() {
            if char.is_numeric() {
                chars.push(char);
            }
        }
        cnt += chars[0..1].parse::<i32>().unwrap() * 10 + 
            chars[chars.len()-1..chars.len()].parse::<i32>().unwrap();
    }

    println!("{cnt}");
    let elapsed = (now.elapsed().as_nanos() as f32) / 1000.0;
    println!("Part 1 took {elapsed} us.")
}


fn part_2() {
    println!("Solving part 2... :)");
    let now: Instant = Instant::now();
    let input: String = fs::read_to_string("input.txt").unwrap();

    let digits = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let mut cnt: u32 = 0;
    for line in input.lines() {
        let mut chars: String = String::new();
        for (i, char) in line.chars().enumerate() {
            if char.is_numeric() {
                chars.push(char);
            } else {
                for pattern in digits.keys() {
                    if line[i..].starts_with(pattern) {
                        let value = digits.get(pattern).unwrap();
                        chars.push_str(value);
                    }
                }
            }
        }
        cnt += chars[0..1].parse::<u32>().unwrap() * 10 + 
            chars[chars.len()-1..chars.len()].parse::<u32>().unwrap();
    }

    println!("{cnt}");
    let elapsed = (now.elapsed().as_nanos() as f32) / 1000.0;
    println!("Part 2 took {elapsed} us.")
}


fn main() {
    println!("Welcome to Advent of Code 2023. Day 1!");
    part_1();
    part_2();
}
