use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("packages/day6/resources/input").unwrap();

    // Part 1
    println!("{}", find_marker_index(&input, 4).unwrap());

    // Part 2
}

fn find_marker_index(data: &str, len: usize) -> Option<usize> {
    let _data: Vec<char> = data.chars().collect();
    let mut iterations = 0;

    for window in _data.windows(len) {
        let mut set: HashSet<char> = HashSet::new();

        for i in 0..len {
            if !set.insert(window[i]) {
                break;
            } else if i == len - 1 {
                return Some(iterations + len);
            }
        }

        iterations += 1;
    }

    // Could not find any marker
    None
}
