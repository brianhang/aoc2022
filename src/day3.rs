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

    let lines: Vec<String> = buf.lines().map(|line| line.unwrap()).collect();
    let groups = lines.chunks(3);

    let total_priority: u32 = groups
        .map(|lines| {
            let mut items: Vec<HashSet<char>> = lines
                .into_iter()
                .map(|line| HashSet::from_iter(line.chars()))
                .collect();

            if let Some((first_items, rest_items)) = items.split_first_mut() {
                for other_items in rest_items {
                    first_items.retain(|item| other_items.contains(item));
                }
                return first_items.iter().map(get_priority).sum();
            }
            return 0;
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
