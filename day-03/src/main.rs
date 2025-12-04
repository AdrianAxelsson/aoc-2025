use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let input_file = File::open("data/input.txt")?;
    let reader = BufReader::new(input_file);

    let mut p1 = 0;
    let mut p2 = 0;

    for line in reader.lines() {
        let line = line?;
        let bank = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();

        p1 += get_joltage(&bank, 2);
        p2 += get_joltage(&bank, 12);
    }

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);

    Ok(())
}

fn get_joltage(bank: &Vec<usize>, battery_count: usize) -> u64 {
    let mut indexes: Vec<usize> = vec![];
    let mut joltages = vec![];

    for b in 0..battery_count {
        let mut current_index = if b == 0 {
            0
        } else {
            indexes.get(b - 1).map(|v| v + 1).unwrap()
        };
        let mut current_max = 0;

        for (i, num) in bank.iter().enumerate() {
            if i < current_index {
                continue;
            }
            if i >= bank.len() - (battery_count - (b + 1)) {
                continue;
            }
            if *num > current_max {
                current_max = *num;
                current_index = i;
            }
        }

        indexes.push(current_index);
    }

    joltages.push(
        indexes
            .iter()
            .map(|x| bank[*x].to_string())
            .collect::<String>()
            .parse::<usize>()
            .unwrap(),
    );
    return joltages.iter().map(|x| *x as u64).sum::<u64>();
}
