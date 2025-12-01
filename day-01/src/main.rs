use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input_file = File::open("data/input.txt").unwrap();
    let reader = BufReader::new(input_file);

    let mut position: i32 = 50;
    let mut zero_pass_count: i32 = 0;
    let mut zero_count: i32 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let (direction, value_str) = line.split_at(1);
        let value: i32 = value_str.parse().unwrap();
        for _ in 0..value {
            position += if direction == "L" { -1 } else { 1 };

            position = match position {
                -1 => 99,
                100 => 0,
                _ => position,
            };

            if position == 0 {
                zero_pass_count += 1
            }
        }
        if position.rem_euclid(100) == 0 {
            zero_count += 1
        }
    }
    println!("Part 1: {}", zero_count);
    println!("Part 2: {}", zero_pass_count);
}
