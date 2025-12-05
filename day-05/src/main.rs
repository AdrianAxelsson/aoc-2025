use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let p1;
    let mut p2 = 0;

    let mut ranges: Vec<(usize, usize)> = vec![];
    let mut ingredients: Vec<usize> = vec![];
    let mut fresh_ingredients = HashSet::new();

    let input_file = File::open("data/input.txt")?;
    let reader = BufReader::new(input_file);

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }
        if line.contains("-") {
            ranges.push(
                line.split_once('-')
                    .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
                    .unwrap(),
            );
        } else {
            ingredients.push(line.parse().unwrap());
        }
    }

    ranges.sort();

    // Part 1
    for &(start, end) in ranges.iter() {
        for &ingredient in ingredients.iter() {
            if ingredient >= start && ingredient <= end {
                fresh_ingredients.insert(ingredient);
            }
        }
    }
    p1 = fresh_ingredients.len();

    // Part 2
    let merged_ranges = merge_ranges(&ranges);

    for &(start, end) in merged_ranges.iter() {
        p2 += (end - start) + 1;
    }

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    Ok(())
}

fn merge_ranges(ranges: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut merged_ranges: Vec<(usize, usize)> = vec![];

    for &(start, end) in ranges {
        match merged_ranges.last_mut() {
            Some((_, prev_end)) if start <= *prev_end + 1 => *prev_end = (*prev_end).max(end),
            _ => merged_ranges.push((start, end)),
        }
    }

    merged_ranges
}
