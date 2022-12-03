use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut total: u64 = 0;
/*
    STEP 1
    for line in reader.lines() {
        let line = line.unwrap();
        let half = line.len() / 2;
        let first = &line[0..half];
        let second = &line[half..];

        let left = HashSet::<u8>::from_iter(first.as_bytes().to_vec().into_iter());
        let right = HashSet::<u8>::from_iter(second.as_bytes().to_vec().into_iter());
        let inter:Vec<_> = left.intersection(&right).collect();

        let obj = inter[0];
        if obj >= &b'a' && obj <= &b'z' {
            total += (obj - &b'a' + 1) as u64;
        } else if obj >= &b'A' && obj <= &b'Z' {
            total += (obj - &b'A' + 27) as u64;
        } else {
            panic!("error!!!");
        }
    }
*/
    // STEP 2
    let mut elf = 0;
    let mut elf1: HashSet<u8> = HashSet::new();
    let mut elf2: HashSet<u8> = HashSet::new();
    for line in reader.lines() {
        let line = line.unwrap();

        elf += 1;
        if elf == 1 {
            elf1 = HashSet::<_>::from_iter(line.as_bytes().to_vec().into_iter());
        } else if elf == 2 {
            elf2 = HashSet::<_>::from_iter(line.as_bytes().to_vec().into_iter());
        } else if elf == 3 {
            let elf3 = HashSet::<_>::from_iter(line.as_bytes().to_vec().into_iter());

            let inter: HashSet<u8> = elf3.intersection(&elf2).cloned().collect();
            let inter: Vec<_> = elf1.intersection(&inter).collect();

            let obj = inter[0];
            if obj >= &b'a' && obj <= &b'z' {
                total += (obj - &b'a' + 1) as u64;
            } else if obj >= &b'A' && obj <= &b'Z' {
                total += (obj - &b'A' + 27) as u64;
            } else {
                panic!("error!!!");
            }

            elf = 0;
        }
    }

    println!("total = {}", total);
}
