use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::Range,
    path::Path,
};

fn main() {
    let input_path = Path::new("src/inputs/day4.txt");
    let file =
        File::open(&input_path).expect(&format!("{} is not a valid file!", input_path.display()));
    let buf = BufReader::new(file);

    let num_pairs = buf
        .lines()
        .filter(|line| {
            let (left, right) = parse_line(line.as_ref().unwrap());
            return range_fully_contains(&left, &right) || range_fully_contains(&right, &left);
        })
        .count();

    println!("{}", num_pairs);
}

fn range_fully_contains(container: &Range<u32>, contained: &Range<u32>) -> bool {
    return container.start <= contained.end && contained.start <= container.end;
}

fn parse_line(line: &String) -> (Range<u32>, Range<u32>) {
    let (left, right) = line.split_once(',').unwrap();
    return (parse_range(left), parse_range(right));
}

fn parse_range(value: &str) -> Range<u32> {
    return value
        .split_once('-')
        .map(|(start, end)| {
            return Range {
                start: start.parse::<u32>().unwrap(),
                end: end.parse::<u32>().unwrap(),
            };
        })
        .unwrap();
}
