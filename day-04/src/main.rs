use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let input_file = File::open("data/input.txt")?;
    let reader = BufReader::new(input_file);

    let mut p1 = 0;
    let mut p2 = 0;

    let mut grid: Vec<Vec<char>> = vec![vec![]];

    for line in reader.lines() {
        let line = line?;
        let temp_vec: Vec<char> = line.chars().collect();
        grid.push(temp_vec);
    }
    grid.remove(0);

    // Part 1
    for (y, row) in grid.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            if *value == '@' {
                if get_neighbor_roll_count(&grid, x, y) < 4 {
                    p1 += 1
                }
            }
        }
    }

    // Part 2
    let mut rolls_removed = true;
    while rolls_removed {
        rolls_removed = false;
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] == '@' {
                    if get_neighbor_roll_count(&grid, x, y) < 4 {
                        p2 += 1;
                        grid[y][x] = '.';
                        rolls_removed = true;
                    }
                }
            }
        }
    }

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);

    Ok(())
}

fn get_neighbor_roll_count(grid: &Vec<Vec<char>>, x: usize, y: usize) -> u32 {
    let directions = vec![
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];
    let mut count = 0;
    for direction in directions {
        if x as i32 + direction.0 < 0 {
            continue;
        }
        if y as i32 + direction.1 < 0 {
            continue;
        }
        if is_roll(
            grid,
            (x as i32 + direction.0) as usize,
            (y as i32 + direction.1) as usize,
        ) {
            count += 1;
        }
    }
    count
}

fn is_roll(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let grid_value = grid
        .get(y)
        .and_then(|row| row.get(x))
        .copied()
        .unwrap_or('.');

    match grid_value {
        '@' => true,
        '.' => false,
        _ => panic!("Invalid char in input"),
    }
}
