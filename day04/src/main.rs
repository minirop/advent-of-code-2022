use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut total: u64 = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        let elves: Vec<&str> = line.split(',').collect();
        let first: Vec<&str> = elves[0].split('-').collect();
        let second: Vec<&str> = elves[1].split('-').collect();

        let fmin = first[0].parse::<i32>().unwrap();
        let fmax = first[1].parse::<i32>().unwrap();
        let smin = second[0].parse::<i32>().unwrap();
        let smax = second[1].parse::<i32>().unwrap();
/*
        // STEP 1
        if (fmin >= smin && fmax <= smax) || (smin >= fmin && smax <= fmax) {
            total += 1;
        }
*/
        // STEP 2
        if (fmin >= smin && fmin <= smax) || (fmax >= smin && fmax <= smax) || (smin >= fmin && smin <= fmax) || (smax >= fmin && smax <= fmax) {
            total += 1;
        }
    }

    println!("total = {}", total);
}
