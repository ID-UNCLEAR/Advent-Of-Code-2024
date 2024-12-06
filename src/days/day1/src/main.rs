use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;

fn main() {
    let mut left_array: Vec<u32> = Vec::new();
    let mut right_array: Vec<u32> = Vec::new();

    let lines = read_file("input.txt").unwrap();

    for line in lines {
        let ints = split(line).unwrap();
        left_array.push(ints.0);
        right_array.push(ints.1);
    }

    let total_sum: u32 = calculate_total_sum(&left_array, &right_array);

    println!("{}", total_sum);
}

fn read_file(file_path: impl AsRef<Path>) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    Ok(lines)
}

fn split(line: String) -> Result<(u32, u32), std::num::ParseIntError> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    Ok((parts[0].parse()?, parts[1].parse()?))
}

fn calculate_total_sum(left_array: &Vec<u32>, right_array: &Vec<u32>) -> u32 {
    left_array
        .iter()
        .map(|&a| {
            let count = right_array.iter().filter(|&&b| b == a).count() as u32;
            a * count
        })
        .sum()
}
