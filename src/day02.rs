use std::error::Error;
use std::fs;

enum RoundResult {
    Win,
    Tie,
    Loss,
}

enum Move {
    Rock,
    Paper,
    Scisors,
}

const FILE_PATH: &str = "inputs/day01.in";

pub fn day02_solutions() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(FILE_PATH)?;
    let rounds_as_string = contents.split("\n").collect::<Vec<&str>>();
    Ok(())
}
