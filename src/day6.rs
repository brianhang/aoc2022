use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

const MARKER_LEN: usize = 4;

fn main() {
    let input_path = Path::new("src/inputs/day6.txt");
    let file =
        File::open(&input_path).expect(&format!("{} is not a valid file!", input_path.display()));
    let buf = BufReader::new(file);

    let mut last_idx: HashMap<char, usize> = HashMap::new();
    let mut start_idx = 0;
    let mut end_idx = 0;

    for line in buf.lines() {
        for ch in line.unwrap().chars() {
            if last_idx.contains_key(&ch) {
                let last_ch_idx = last_idx[&ch];
                if last_ch_idx >= start_idx {
                    start_idx = last_ch_idx + 1;
                }
            }

            if (end_idx - start_idx + 1) == MARKER_LEN {
                println!("{}", end_idx + 1);
                return;
            }

            last_idx.insert(ch, end_idx);
            end_idx += 1;
        }
    }
}
