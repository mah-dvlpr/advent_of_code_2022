use std::collections::HashSet;
use std::fs;
use std::io::{BufRead, BufReader};

fn main() {
    let file = fs::File::open("packages/day3/resources/input_real").unwrap();
    let lines = BufReader::new(file).lines();
    let mut sum = 0;

    for line in lines {
        sum += duplicate_value(&line.unwrap());
    }

    println!("{}", sum);
}

fn duplicate_value(line: &str) -> usize {
    let (left, right) = line.split_at(line.len() / 2);
    let mut set = HashSet::new();
    let mut sum = 0;

    for c in left.chars() {
        set.insert(c);
    }

    for c in right.chars() {
        if set.contains(&c) {
            let offset;
            if (c as u8) < (b'a') {
                offset = b'A' - 26;
            } else {
                offset = b'a';
            }
            sum += (c as u8 - offset) as usize + 1;
            break;
        }
    }

    sum
}
