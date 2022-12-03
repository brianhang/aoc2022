use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    let input_path = Path::new("src/inputs/day3.txt");
    let file =
        File::open(&input_path).expect(&format!("{} is not a valid file!", input_path.display()));
    let buf = BufReader::new(file);

    let total_priority: u32 = buf
        .lines()
        .map(|line| {
            let line = line.expect("Failed to read a line!");
            let mut items1: Vec<char> = line.chars().collect();
            let items2 = items1.split_off(items1.len() / 2);

            let items1_set: HashSet<char> = HashSet::from_iter(items1);
            let items2_set: HashSet<char> = HashSet::from_iter(items2);

            let dupes = items1_set.intersection(&items2_set);

            let dupe_priority: u32 = dupes.map(get_priority).sum();
            return dupe_priority;
        })
        .sum();

    println!("{}", total_priority);
}

fn get_priority(item: &char) -> u32 {
    match *item {
        'a'..='z' => (*item as u32) - ('a' as u32) + 1,
        'A'..='Z' => (*item as u32) - ('A' as u32) + 27,
        _ => panic!("invalid item: {}", item),
    }
}
