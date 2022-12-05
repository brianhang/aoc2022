#[macro_use]
extern crate lazy_static;

use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use regex::Regex;

#[derive(Clone, Copy)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

fn main() {
    let input_path = Path::new("src/inputs/day5.txt");
    let file =
        File::open(&input_path).expect(&format!("{} is not a valid file!", input_path.display()));
    let buf = BufReader::new(file);

    let (mut stacks, moves) = parse_input(buf);
    let mut crane: VecDeque<char> = VecDeque::new();

    for Move { count, from, to } in moves {
        for _ in 0..count {
            let from_crate = stacks[from].pop_back().unwrap();
            crane.push_back(from_crate);
        }
        for _ in 0..count {
            let from_crate = crane.pop_back().unwrap();
            stacks[to].push_back(from_crate);
        }
    }

    let tops: String = stacks.iter().filter_map(|s| s.back()).collect();

    println!("{}", tops);
}

fn parse_input(buf: BufReader<File>) -> (Vec<VecDeque<char>>, Vec<Move>) {
    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    let mut moves: Vec<Move> = Vec::new();

    for line in buf.lines() {
        let line = line.unwrap();

        if line.starts_with("move") {
            moves.push(parse_move(&line));
        } else {
            let crate_and_stack_idx = parse_crates(&line);
            for (crate_name, stack_idx) in crate_and_stack_idx {
                while stack_idx >= stacks.len() {
                    stacks.push(VecDeque::new());
                }

                stacks[stack_idx].push_front(crate_name);
            }
        }
    }

    return (stacks, moves);
}

fn parse_move(line: &String) -> Move {
    lazy_static! {
        static ref MOVE_REGEX: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    }

    let args = MOVE_REGEX.captures(line).unwrap();
    Move {
        count: args
            .get(1)
            .map_or(0, |n| n.as_str().parse::<usize>().unwrap()),
        from: args
            .get(2)
            .map_or(0, |n| n.as_str().parse::<usize>().unwrap() - 1),
        to: args
            .get(3)
            .map_or(0, |n| n.as_str().parse::<usize>().unwrap() - 1),
    }
}

fn parse_crates(line: &String) -> Vec<(char, usize)> {
    return line
        .chars()
        .enumerate()
        .filter_map(|(ch_idx, ch)| {
            if !ch.is_alphabetic() {
                return None;
            }

            return Some((ch, (ch_idx - 1) / 4));
        })
        .collect();
}
