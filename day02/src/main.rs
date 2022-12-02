use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut total = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        let opponent = line.as_bytes()[0];
        let me = line.as_bytes()[2];
/*
        // STEP 1
        if opponent == 65 && me == 88 {
            total += 1 + 3;
        } else if opponent == 65 && me == 89 {
            total += 2 + 6;
        } else if opponent == 65 && me == 90 {
            total += 3;
        } else if opponent == 66 && me == 88 {
            total += 1;
        } else if opponent == 66 && me == 89 {
            total += 2 + 3;
        } else if opponent == 66 && me == 90 {
            total += 3 + 6;
        } else if opponent == 67 && me == 88 {
            total += 1 + 6;
        } else if opponent == 67 && me == 89 {
            total += 2;
        } else if opponent == 67 && me == 90 {
            total += 3 + 3;
        } else {
            panic!("LOL");
        }
*/
        // STEP 2
        if opponent == 65 && me == 88 {
            total += 3;
        } else if opponent == 65 && me == 89 {
            total += 1 + 3;
        } else if opponent == 65 && me == 90 {
            total += 2 + 6;
        } else if opponent == 66 && me == 88 {
            total += 1;
        } else if opponent == 66 && me == 89 {
            total += 2 + 3;
        } else if opponent == 66 && me == 90 {
            total += 3 + 6;
        } else if opponent == 67 && me == 88 {
            total += 2;
        } else if opponent == 67 && me == 89 {
            total += 3 + 3;
        } else if opponent == 67 && me == 90 {
            total += 1 + 6;
        } else {
            panic!("LOL");
        }
    }

    println!("{}", total);
}
