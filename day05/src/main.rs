use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut stacks: Vec<Vec<char>> = vec![];
    let mut build = false;
    for line in reader.lines() {
        let line = line.unwrap();
        let stack_count = line.len() / 4;
        let bytes = line.as_bytes();

        if stacks.len() == 0 {
            for _ in 0..(stack_count+2) {
                stacks.push(vec![]);
            }
        }

        if !build {
            if bytes[1] == b'1' {
                build = true;
            } else {
                for i in 0..(stack_count+1) {
                    let val = bytes[i * 4 + 1];
                    if val != 32 {
                        stacks[i+1].insert(0, val as char);
                    }
                }
            }
        } else if line.len() > 0 {
            if let Some((count, from, to)) = parse_line(&line) {
/*
                // STEP 1
                for _ in 0..count {
                    if let Some(val) = stacks[from].pop() {
                        stacks[to].push(val);
                    } else {
                        break;
                    }
                }
*/
                // STEP 2
                let cur_size = stacks[from].len();
                let start = if count >= cur_size { 0 } else { cur_size - count };
                let mut u: Vec<_> = stacks[from].drain(start..).collect();
                stacks[to].append(&mut u);
            }
        }
    }

    for stack in stacks {
        if stack.len() == 0 {
            // skip first stack
        } else {
            print!("{}", *stack.last().unwrap());
        }
    }
    print!("\n");
}

fn parse_line(s: &str) -> Option<(usize, usize, usize)> {
    let r = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let caps = r.captures(s)?;
    let a = caps.get(1)?.as_str().parse().ok()?;
    let b = caps.get(2)?.as_str().parse().ok()?;
    let c = caps.get(3)?.as_str().parse().ok()?;
    Some((a, b, c))
}
