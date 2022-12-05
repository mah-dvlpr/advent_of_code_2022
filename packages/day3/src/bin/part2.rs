use std::collections::HashSet;
use std::fs;
use std::io::{BufRead, BufReader};

fn main() {
    let file = fs::read_to_string("packages/day3/resources/input_simple_part2").unwrap();
    let lines: Vec<&str> = file.split('\n').collect();
    let mut sum = 0;

    for line in &lines {
        sum += duplicate_value(&line);
    }

    println!("{}", sum);

    let mut count = 1;
    let mut dat = String::new();
    for line in &lines {
        dat.push_str(line);

        if count % 3 == 0 {
            count = 1;
        } else {
            count += 1;
        }
    }
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
