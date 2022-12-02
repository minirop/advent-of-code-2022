use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut current_elf = 0;
    let mut elves = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            elves.push(current_elf);
            current_elf = 0;
        } else {
            current_elf += line.parse::<i32>().unwrap();
        }
    }

    elves.push(current_elf);
    elves.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
    let total = elves[0] + elves[1] + elves[2];

    println!("{:?}", total);
}
