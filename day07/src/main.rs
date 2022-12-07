use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut folders: Vec<String> = vec![];
    let mut sizes = HashMap::new();
    let mut total_used_space = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let data: Vec<_> = line.split(' ').collect();

        if data[0] == "$" {
            match data[1] {
                "cd" => {
                    match data[2] {
                        ".." => { folders.pop(); },
                        "/" => folders.clear(),
                        folder => folders.push(folder.to_string()),
                    }
                },
                "ls" => {},
                _ => panic!("ERROR"),
            }
        } else if data[0] == "dir" {
        } else {
            let mut folders_path: Vec<String> = vec![];
            let size = data[0].parse::<u32>().unwrap();

            total_used_space += size;

            for dir in &folders {
                folders_path.push(dir.to_string());
                let full_path = folders_path.join("/");

                if let Some(val) = sizes.get_mut(&full_path) {
                    *val += size;
                } else {
                    sizes.insert(full_path, size);
                }
            }
        }
    }

/*
    // STEP 1
    let mut total = 0;
    for (_, v) in &sizes {
        if v <= &100000 {
            total += v;
        }
    }

    println!("{:?}", total);
*/
    // STEP 2
    let max_space = 40000000;
    let to_free = total_used_space - max_space;

    let mut min_size = max_space;
    let mut candidate = 0;
    for (_, v) in &sizes {
        if v >= &to_free {
            let new_min_size = v - to_free;
            if new_min_size < min_size {
                min_size = new_min_size;
                candidate = *v;
            }
        }
    }

    println!("{:?}", candidate);
}
