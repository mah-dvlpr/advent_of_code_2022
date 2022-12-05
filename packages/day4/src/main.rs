use std::{
    fs,
    ops::{Deref, RangeInclusive},
    str::Split,
};

fn main() {
    let input = fs::read_to_string("packages/day4/resources/input").unwrap();
    let input: Vec<&str> = input.split("\n").collect();
    let mut sum = 0;

    // Part 1
    for line in &input {
        let s: Vec<&str> = split_group(*line).collect();
        let left = str_to_inclusive_range(s[0]);
        let right = str_to_inclusive_range(s[1]);
        if is_fully_contained(&left, &right) {
            sum += 1;
        }
    }

    println!("Fully contained sectors pairs: {}", sum);

    // Part 2
    sum = 0;

    for line in &input {
        let s: Vec<&str> = split_group(*line).collect();
        let left = str_to_inclusive_range(s[0]);
        let right = str_to_inclusive_range(s[1]);
        if let Some(_) = get_overlap(&left, &right) {
            sum += 1;
        }
    }

    println!("Number of overlapping sectors: {}", sum);
}

fn split_group<'a>(s: &'a str) -> Split<'a, &str> {
    s.split(",")
}

fn str_to_inclusive_range(s: &str) -> RangeInclusive<usize> {
    let elements: Vec<&str> = s.split('-').collect();
    let left = elements[0].parse::<usize>().unwrap();
    let right = elements[1].parse::<usize>().unwrap();
    left..=right
}

fn is_fully_contained(left: &RangeInclusive<usize>, right: &RangeInclusive<usize>) -> bool {
    fn fully_contained(left: &RangeInclusive<usize>, right: &RangeInclusive<usize>) -> bool {
        for e in left.clone() {
            if !right.contains(&e) {
                break;
            } else if left.clone().last().unwrap() == e {
                return true;
            }
        }
        false
    }
    fully_contained(left, right) || fully_contained(right, left)
}

fn get_overlap(
    left: &RangeInclusive<usize>,
    right: &RangeInclusive<usize>,
) -> Option<RangeInclusive<usize>> {
    fn get_overlap(
        left: &RangeInclusive<usize>,
        right: &RangeInclusive<usize>,
    ) -> Option<RangeInclusive<usize>> {
        if left.end() >= right.start() {
            return Some(*left.start()..=*right.end());
        }
        None
    }

    if left.start() < right.start() {
        return get_overlap(left, right);
    }

    get_overlap(right, left)
}
