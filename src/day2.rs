use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Copy, Clone)]
enum Choice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Copy, Clone)]
enum Score {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

fn main() {
    let input_path = Path::new("src/inputs/day2.txt");
    let file =
        File::open(&input_path).expect(&format!("{} is not a valid file!", input_path.display()));
    let buf = BufReader::new(file);

    let total_score: i32 = buf
        .lines()
        .map(|line| {
            let line = line.expect("Failed to read a line!");
            let (opponent_raw, score_raw) = line
                .split_once(' ')
                .expect(&format!("Expected space separated parts: {}", line));
            let opponent = parse_choice(opponent_raw);
            let score = parse_score(score_raw);
            let choice = get_desired_choice(opponent, score);

            return (score as i32) + (choice as i32);
        })
        .sum();

    println!("{}", total_score);
}

fn parse_choice(raw_choice: &str) -> Choice {
    match raw_choice {
        "A" => Choice::Rock,
        "B" => Choice::Paper,
        "C" => Choice::Scissors,
        _ => panic!("Invalid choice: {}", raw_choice),
    }
}

fn parse_score(raw_result: &str) -> Score {
    match raw_result {
        "X" => Score::Loss,
        "Y" => Score::Draw,
        "Z" => Score::Win,
        _ => panic!("Invalid result: {}", raw_result),
    }
}

fn get_desired_choice(opponent: Choice, score: Score) -> Choice {
    match (opponent, score) {
        (Choice::Rock, Score::Loss) => Choice::Scissors,
        (Choice::Scissors, Score::Loss) => Choice::Paper,
        (Choice::Paper, Score::Loss) => Choice::Rock,

        (Choice::Rock, Score::Win) => Choice::Paper,
        (Choice::Scissors, Score::Win) => Choice::Rock,
        (Choice::Paper, Score::Win) => Choice::Scissors,

        _ => opponent,
    }
}
