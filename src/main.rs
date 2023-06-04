use std::io;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_input("./input.md") {
        println!("{}", get_largest_group(lines))
    }
}

fn get_largest_group(lines: Lines<BufReader<File>>) -> i32 {
    let mut largest = 0;
    let mut current = 0;
    lines.flatten()
        .for_each(|l| {
            match l.as_str() {
                "" => {
                    largest = largest.max(current);
                    current = 0;
                }
                _ => {
                    current += l.parse::<i32>().unwrap();
                }
            }
        });
    largest
}

fn read_input<P>(filename: P) -> Result<Lines<BufReader<File>>, io::Error>
    where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
