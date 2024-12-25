use std::env;
use std::io::{self, BufRead};

use std::fs::File;
use std::collections::VecDeque;

// Parses the input into a queue of rows of numbers
fn read_and_parse(path: &str) -> Result<VecDeque<Vec<u32>>, io::Error> {
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut parsed_numbers: Vec<Vec<u32>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<u32> = line
            .split('\t') // Split on tabs
            .filter_map(|s| s.trim().parse::<u32>().ok())
            .collect();

        parsed_numbers.push(numbers);
    }

    Ok(parsed_numbers.into())
}

fn find_max_min(slice: &[u32]) -> Option<(u32, u32)> {
    if slice.len() < 2 {
        return None;
    }

    let mut min = u32::MAX;
    let mut max = u32::MIN;

    for &num in slice {
        if num < min {
            min = num;
        }
        if num > max {
            max = num;
        }
    }

    Some((max, min))
}

fn find_divisible(vec: &[u32]) -> Option<(u32, u32)> {
    if vec.len() < 2 {
        return None;
    }

    let head = vec[0];
    let tail = &vec[1..];

    if let Some(&divisor) = tail.iter().find(|&&x| head % x == 0 || x % head == 0) {
        return Some((head, divisor));
    }

    find_divisible(tail)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];

    let rows: VecDeque<Vec<u32>> = read_and_parse(file_path)?;

    // Part 1: Compute differences between max and min for each row
    let diffs: Vec<u32> = rows
        .iter()
        .filter_map(|row| find_max_min(row.as_slice()))
        .map(|(x, y)| x - y)
        .collect();

    let part1: u32 = diffs.iter().sum();

    // Part 2: Compute divisions
    let divisors: Vec<u32> = rows
        .iter()
        .filter_map(|row| find_divisible(row.as_slice()))
        .map(|(x, y)| if x > y {x / y} else {y / x})
        .collect();

    let part2: u32 = divisors.iter().sum();

    println!("Part 1: {}\nPart 2: {}", part1, part2);

    Ok(())
}
