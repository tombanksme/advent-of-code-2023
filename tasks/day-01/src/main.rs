use std::fs::File;
use std::io;
use std::io::BufRead;

/// @TODO: Second part of task using nom?

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let result: u32 = io::BufReader::new(file)
        .lines()
        .into_iter()
        .map(|line| {
            line.unwrap()
                .chars()
                .filter(|char| char.is_numeric())
                .map(|char| char.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .map(|nums| {
            let first = nums.first().unwrap();
            let last = nums.last().unwrap();

            return (first * 10) + last;
        })
        .sum();

    Ok(())
}
