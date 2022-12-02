use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Copy, Clone)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

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
            let (opponent_raw, me_raw) = line
                .split_once(' ')
                .expect(&format!("Expected space separated parts: {}", line));
            let opponent = parse_choice(opponent_raw);
            let me = parse_choice(me_raw);

            return (get_round_score(opponent, me) as i32) + get_choice_score(me);
        })
        .sum();

    println!("{}", total_score);
}

fn parse_choice(raw_choice: &str) -> Choice {
    match raw_choice {
        "A" | "X" => Choice::Rock,
        "B" | "Y" => Choice::Paper,
        "C" | "Z" => Choice::Scissors,
        _ => panic!("Invalid choice: {}", raw_choice),
    }
}

fn get_round_score(opponent: Choice, me: Choice) -> Score {
    match (opponent, me) {
        (Choice::Rock, Choice::Scissors) => Score::Loss,
        (Choice::Scissors, Choice::Paper) => Score::Loss,
        (Choice::Paper, Choice::Rock) => Score::Loss,

        (Choice::Rock, Choice::Paper) => Score::Win,
        (Choice::Scissors, Choice::Rock) => Score::Win,
        (Choice::Paper, Choice::Scissors) => Score::Win,

        _ => Score::Draw,
    }
}

fn get_choice_score(choice: Choice) -> i32 {
    match choice {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3,
    }
}
