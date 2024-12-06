use std::path::Path;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() {
    let mut left_array: Vec<u32> = Vec::new();
    let mut right_array: Vec<u32> = Vec::new();

    // Read the lines from the input file
    let lines = read_file("input.txt").unwrap();

    // For each line, split it into two integers and push them into the respective arrays
    for line in lines {
        let ints = split(line).unwrap();
        left_array.push(ints.0);
        right_array.push(ints.1);
    }

    // Calculate the total product sum
    let total_sum: u32 = calculate_total_sum(&left_array, &right_array);

    // Print the total sum
    println!("{}", total_sum);
}

// Function to read the input file and return the lines as a vector of strings
fn read_file(file_path: impl AsRef<Path>) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    Ok(lines)
}

// Function to split a line into two integers
fn split(line: String) -> Result<(u32, u32), std::num::ParseIntError> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    Ok((parts[0].parse()?, parts[1].parse()?))
}

// Function to calculate the total sum of products
fn calculate_total_sum(left_array: &Vec<u32>, right_array: &Vec<u32>) -> u32 {
    left_array.iter()
        .map(|&a| {
            // Count how many times `a` appears in the right array
            let count = right_array.iter().filter(|&&b| b == a).count() as u32;
            a * count  // Multiply the left value by the count from the right array
        })
        .sum()  // Sum all the products
}
