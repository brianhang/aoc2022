use std::{
    cell::RefCell,
    collections::HashMap,
    fs,
    io::{BufRead, BufReader},
    path::Path,
    rc::Rc,
};

struct File {
    size: usize,
}

struct Dir {
    name: String,
    entries: HashMap<String, Rc<RefCell<Entry>>>,
}

enum Entry {
    File(Rc<RefCell<File>>),
    Dir(Rc<RefCell<Dir>>),
}

fn main() {
    let input_path = Path::new("src/inputs/day7.txt");
    let file = fs::File::open(&input_path)
        .expect(&format!("{} is not a valid file!", input_path.display()));
    let buf = BufReader::new(file);
    let root_dir = Rc::new(RefCell::new(make_dir("/".to_string())));

    parse_file_system(buf, &root_dir);

    let (_, answer) = total_size(&root_dir.clone(), 100_000);
    println!("{}", answer);
}

fn total_size(dir: &Rc<RefCell<Dir>>, max_size: usize) -> (usize, usize) {
    let entries = &dir.borrow().entries;
    let mut answer: usize = 0;
    let mut dir_size: usize = 0;

    for entry in entries.values() {
        match &*entry.borrow() {
            Entry::Dir(sub_dir) => {
                let (sub_dir_size, sub_answer) = total_size(&sub_dir, max_size);
                answer += sub_answer;
                dir_size += sub_dir_size;
            }
            Entry::File(file) => dir_size += file.borrow().size,
        }
    }

    println!("{} -> {}", dir.borrow().name, dir_size);

    if dir_size <= max_size {
        answer += dir_size;
    }

    return (dir_size, answer);
}

fn parse_file_system(buf: BufReader<fs::File>, root_dir: &Rc<RefCell<Dir>>) {
    let mut dir_stack: Vec<Rc<RefCell<Dir>>> = Vec::new();
    dir_stack.push(Rc::clone(&root_dir));

    let mut is_listing_entries = false;
    for line in buf.lines() {
        let line = line.unwrap();

        if let Some(command) = line.strip_prefix("$ ") {
            is_listing_entries = false;

            let args: Vec<&str> = command.split_whitespace().collect();
            match args.as_slice() {
                ["cd", dir] => match dir {
                    &"/" => dir_stack.truncate(1),

                    &".." => _ = dir_stack.pop(),

                    _ => {
                        let entry = &dir_stack
                            .last()
                            .unwrap()
                            .borrow()
                            .entries
                            .get(&dir.to_string())
                            .expect(&format!("{} is not a subdirectory", dir))
                            .clone();

                        match &*entry.borrow() {
                            Entry::Dir(sub_dir) => dir_stack.push(sub_dir.clone()),
                            _ => panic!("invalid sub dir! {}", dir),
                        };
                    }
                },

                ["ls"] => is_listing_entries = true,

                _ => panic!("not implemented"),
            };
        } else if is_listing_entries {
            let entries: Vec<&str> = line.split_whitespace().collect();
            let dir_or_size = entries[0];
            let name = entries[1];
            let entry: Rc<RefCell<Entry>>;

            if dir_or_size == "dir" {
                let sub_dir = Rc::new(RefCell::new(make_dir(name.to_string())));
                entry = Rc::new(RefCell::new(Entry::Dir(sub_dir)));
            } else {
                let file = Rc::new(RefCell::new(File {
                    size: dir_or_size.parse::<usize>().unwrap(),
                }));
                entry = Rc::new(RefCell::new(Entry::File(file)));
            }
            dir_stack
                .last()
                .unwrap()
                .borrow_mut()
                .entries
                .insert(name.to_string(), entry);
        }
    }
}

fn make_dir(name: String) -> Dir {
    return Dir {
        name,
        entries: HashMap::new(),
    };
}
