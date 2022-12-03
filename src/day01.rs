use std::error::Error;
use std::fs;

const FILE_PATH: &str = "inputs/day01.in";

pub fn get_calorie_sums() -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string(FILE_PATH)?;
    let mut sums = file
        .split("\n\n")
        .map(|line| {
            line.split("\n")
                .map(|num_str| num_str.parse::<u32>().unwrap())
                .fold(0, |acc, num| acc + num)
        })
        .collect::<Vec<u32>>();
    sums.sort_by(|a, b| b.cmp(a));
    println!("Part 1: {}", sums[0]);
    println!("Part 2: {}", sums[0] + sums[1] + sums[2]);
    Ok(())
}
