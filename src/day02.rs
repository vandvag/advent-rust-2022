use std::error::Error;
use std::fs;

enum RoundResult {
    Win,
    Tie,
    Loss,
}

#[derive(PartialEq, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scisors,
}

const FILE_PATH: &str = "inputs/day02.in";

pub fn day02_solutions() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(FILE_PATH)?;
    let part1 = contents.split("\n").fold(0, |acc, round_str| {
        // The round string is always "A X"
        let opponent_move = opponent_char_to_move(round_str.chars().nth(0).unwrap()).unwrap();
        let my_move = my_char_to_move(round_str.chars().nth(2).unwrap()).unwrap();
        let round_result = find_round_result(&my_move, &opponent_move);
        acc + calculate_round_score(&my_move, &round_result)
    });
    let part2 = contents.split("\n").fold(0, |acc, round_str| {
        // The round string is always "A X"
        let opponent_move = opponent_char_to_move(round_str.chars().nth(0).unwrap()).unwrap();
        let round_result = my_char_to_result(round_str.chars().nth(2).unwrap()).unwrap();
        let my_move = round_result_to_move(&round_result, opponent_move);
        acc + calculate_round_score(&my_move, &round_result)
    });
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    Ok(())
}

fn my_char_to_move(my_char: char) -> Option<&'static Move> {
    match my_char {
        'X' => Some(&Move::Rock),
        'Y' => Some(&Move::Paper),
        'Z' => Some(&Move::Scisors),
        _ => None,
    }
}

fn opponent_char_to_move(opponent_char: char) -> Option<&'static Move> {
    match opponent_char {
        'A' => Some(&Move::Rock),
        'B' => Some(&Move::Paper),
        'C' => Some(&Move::Scisors),
        _ => None,
    }
}

fn my_char_to_result(my_char: char) -> Option<&'static RoundResult> {
    match my_char {
        'X' => Some(&RoundResult::Loss),
        'Y' => Some(&RoundResult::Tie),
        'Z' => Some(&RoundResult::Win),
        _ => None,
    }
}

fn round_result_to_move(round_result: &RoundResult, opponent_move: &Move) -> Move {
    match round_result {
        &RoundResult::Win => move_loses_to_move(opponent_move),
        &RoundResult::Loss => move_loses_to_move(&move_loses_to_move(opponent_move)),
        &RoundResult::Tie => *opponent_move,
    }
}

fn move_to_points(mv: &Move) -> u32 {
    match mv {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scisors => 3,
    }
}

fn result_to_points(res: &RoundResult) -> u32 {
    match res {
        RoundResult::Win => 6,
        RoundResult::Loss => 0,
        RoundResult::Tie => 3,
    }
}

fn move_loses_to_move(mv: &Move) -> Move {
    match mv {
        Move::Rock => Move::Paper,
        Move::Paper => Move::Scisors,
        Move::Scisors => Move::Rock,
    }
}

fn find_round_result(my_move: &Move, opponent_move: &Move) -> RoundResult {
    if my_move == opponent_move {
        return RoundResult::Tie;
    }
    if move_loses_to_move(my_move) == *opponent_move {
        return RoundResult::Loss;
    } else {
        return RoundResult::Win;
    }
}

fn calculate_round_score(mv: &Move, res: &RoundResult) -> u32 {
    result_to_points(res) + move_to_points(mv)
}
