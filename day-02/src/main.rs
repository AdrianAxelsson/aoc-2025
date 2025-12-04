use std::{fs::File, io::Read};

fn main() -> std::io::Result<()> {
    let mut input_file = File::open("data/input.txt")?;
    let mut input_content = String::new();
    input_file.read_to_string(&mut input_content)?;
    let trimmed_input = input_content.trim();
    let ranges: Vec<_> = trimmed_input.split(',').collect();

    let mut p2_sum = 0;
    let mut p1_sum = 0;

    for range in ranges {
        let range_vec: Vec<usize> = range.split('-').map(|r| r.parse().unwrap()).collect();
        for i in range_vec[0]..=range_vec[1] {
            let i_string = i.to_string();
            if i_string.len() < 2 {
                continue;
            }

            let (left, right) = i_string.split_at(i_string.len() / 2);
            if left == right {
                p1_sum += i;
            }

            let first_char = i_string.chars().nth(0).unwrap();
            if i_string.chars().all(|c| c == first_char) {
                p2_sum += i;
                continue;
            }

            let chunk_sizes = get_divisors(i_string.len());
            for chunk_size in chunk_sizes {
                let compare: String = i_string.chars().take(chunk_size).collect();
                let mut out = Vec::new();
                let mut buf = String::new();
                for (i, char) in i_string.chars().enumerate() {
                    if i % chunk_size == 0 && !buf.is_empty() {
                        out.push(std::mem::take(&mut buf));
                    }
                    buf.push(char);
                }
                if !buf.is_empty() {
                    out.push(buf);
                }
                if out.into_iter().all(|x| x == compare) {
                    p2_sum += i_string.parse::<usize>().unwrap();
                    break;
                }
            }
        }
    }
    println!("Part 1: {}", p1_sum);
    println!("Part 2: {}", p2_sum);
    Ok(())
}

fn get_divisors(num: usize) -> Vec<usize> {
    let mut divisors: Vec<usize> = vec![];
    for i in 2..=num - 1 {
        if num % i == 0 {
            divisors.push(i);
        }
    }
    return divisors;
}
