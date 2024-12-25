use std::env;
use std::fs;
use std::io;

fn parse(bytes: &[u8]) -> Vec<u8> {
    // Remove trailing newline if it exists
    let trimmed = if let Some(&b'\n') = bytes.last() {
        &bytes[..bytes.len() - 1]
    } else {
        bytes
    };

    // Convert bytes to digits
    trimmed.iter().map(|&b| b - b'0').collect()
}

fn solve(digits: &[u8]) -> (u32, u32) {
    let mut sum1: u32 = 0;
    let mut sum2: u32 = 0;
    let half_sz = digits.len() / 2;

    for i in 0..digits.len() {
        // Part 1: Compare to next (wrapping around)
        let next_index = (i + 1) % digits.len();
        if digits[i] == digits[next_index] {
            sum1 += digits[i] as u32;
        }

        // Part 2: Halfway-around comparison
        let halfway_index = (i + half_sz) % digits.len();
        if digits[i] == digits[halfway_index] {
            sum2 += digits[i] as u32;
        }
    }

    (sum1, sum2)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];

    let contents = fs::read(file_path)?;
    let digits = parse(&contents);

    let (sum1, sum2) = solve(&digits);

    println!("Part 1: {}\nPart 2: {}", sum1, sum2);

    Ok(())
}
