use std::env;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];

    let contents = fs::read(file_path)?;

    let bytes = &contents[..contents.len() - 1]; // drop the last byte '\n'

    let mut sum1 = 0;
    let mut sum2 = 0;
    let half_sz = bytes.len() / 2;

    for i in 0..bytes.len() {

        // Part 1: Sequential comparison
        if i < bytes.len() - 1 {
            if bytes[i] == bytes[i + 1] {
                sum1 += (bytes[i] - b'0') as i32;
            }
        } else {
            // Special case: Compare last with first
            if bytes[i] == bytes[0] {
                sum1 += (bytes[i] - b'0') as i32;
            }
        }

        // Part 2: Halfway-around comparison (requires modulo)
        if bytes[i] == bytes[(i + half_sz) % bytes.len()] {
            sum2 += (bytes[i] - b'0') as i32;
        }
    }

    println!("Part 1: {}\nPart 2: {}", sum1, sum2);

    Ok(())
}
