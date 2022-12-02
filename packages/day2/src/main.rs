use std::fs;

fn main() {
    let data = fs::read_to_string("packages/day2/resources/input_simple").expect("Failed to read file.");
}