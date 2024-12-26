use std::env;
use std::io;
use std::collections::HashMap;

fn spiral_step(n: u32) -> (i32, i32) {
    if n == 0 {
        return (0, 0); // The center of the spiral
    }

    let g = (n as f64).sqrt().floor() as i32;
    let r = (g + g % 2) / 2; // Radius of the ring
    let q = 4 * r.pow(2);    // Maximum value in the current ring
    let d = n as i32 - q;    // Distance from the max value

    if n <= (q - 2 * r) as u32 {
        (d + 3 * r, r)
    } else if n <= q as u32 {
        (r, -d - r)
    } else if n <= (q + 2 * r) as u32 {
        (r - d, -r)
    } else {
        (-r, d - 3 * r)
    }
}

fn solve_part_1(i: u32) -> i32 {
    let (x, y) = spiral_step(i - 1);
    x.abs() + y.abs()
}

fn solve_part_2(target: u32) -> i32 {
    find_first_larger(target as i32)
}

fn find_first_larger(target: i32) -> i32 {
    let mut grid = HashMap::new();
    grid.insert((0, 0), 1); // Initialize grid center
    go(1, &mut grid, target)
}

fn go(n: i32, grid: &mut HashMap<(i32, i32), i32>, target: i32) -> i32 {
    let (x, y) = spiral_step(n as u32);

    let directions = vec![
        (-1, -1), (0, -1), (1, -1),
        (-1, 0),          (1, 0),
        (-1, 1),  (0, 1),  (1, 1),
    ];

    let sum_neighbors: i32 = directions
        .iter()
        .map(|&(dx, dy)| grid.get(&(x + dx, y + dy)).unwrap_or(&0))
        .sum();

    grid.insert((x, y), sum_neighbors);

    if sum_neighbors > target {
        sum_neighbors
    } else {
        go(n + 1, grid, target)
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <number>", args[0]);
        std::process::exit(1);
    }

    let number: u32 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: '{}' is not a valid number.", args[1]);
            std::process::exit(1);
        }
    };

    let part1 = solve_part_1(number);
    let part2 = solve_part_2(number);

    println!("Part1: {}\nPart2: {}", part1, part2);

    Ok(())
}
