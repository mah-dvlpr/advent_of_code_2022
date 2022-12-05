use std::{
    cmp::{max, min},
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

    // Part 1.1
    sum = 0;

    for line in &input {
        let s: Vec<&str> = split_group(*line).collect();
        let left = str_to_inclusive_range(s[0]);
        let right = str_to_inclusive_range(s[1]);
        if let Some(x) = get_overlap(&left, &right) {
            if x == left || x == right {
                sum += 1;
            }
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
    fn are_overlapping(left: &RangeInclusive<usize>, right: &RangeInclusive<usize>) -> bool {
        max(left.start(), right.start()) <= min(left.end(), right.end())
    }

    if !are_overlapping(left, right) {
        return None;
    }

    let largest_min = max(left.start(), right.start());
    let smallest_max = min(left.end(), right.end());

    Some(*largest_min..=*smallest_max)
}
