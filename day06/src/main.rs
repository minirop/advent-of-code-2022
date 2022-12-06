use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    let message_start_length = 14;
    let bytes = line.as_bytes();

    'outer: for i in 0..(line.len() - message_start_length) {
        for x in i..(i+message_start_length - 1) {
            for y in (x + 1)..(i+message_start_length) {
                if bytes[x] == bytes[y] {
                    continue 'outer;
                }
            }
        }
        println!("{}", i + message_start_length);
        break;
    }
}
