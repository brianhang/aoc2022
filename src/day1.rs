use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    process,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("An input file must be provided");
        process::exit(1);
    }

    let input_path = Path::new(&args[1]);
    let file =
        File::open(&input_path).expect(&format!("{} is not a valid file!", input_path.display()));
    let buf = BufReader::new(file);

    let mut cur_sum = 0;
    let mut all_calories = Vec::new();

    buf.lines().for_each(|line| {
        let line = line.expect("Failed to read a line!");

        if line.is_empty() {
            all_calories.push(cur_sum);
            cur_sum = 0;
        } else {
            cur_sum += line.parse::<i32>().unwrap();
        }
    });

    all_calories.sort();

    let len = all_calories.len();
    println!(
        "{}",
        all_calories[len - 3] + all_calories[len - 2] + all_calories[len - 1]
    );
}
