use std::fs;
use std::io::{BufRead, BufReader};

fn main() {
    let input = fs::read_to_string("packages/day2/resources/input").unwrap();
    let lines: Vec<&str> = input.split("\n").collect();
    let mut points: usize = 0;

    // Part 1
    for line in &lines {
        let line: Vec<char> = line.chars().collect();

        let their = Hand::get_type_part1(&line[0]);
        let our = Hand::get_type_part1(&line[2]);

        points += get_points(&get_result(&our, &their));
        points += Hand::get_points(&our);
    }

    println!("Total score: {}", points);

    // Part 2
    points = 0;
    for line in &lines {
        let line: Vec<char> = line.chars().collect();

        let action = &line[2];
        let their = Hand::get_type_part2(&line[0]);
        let our = match action {
            'X' => their.get_losing_hand(),
            'Y' => their,
            'Z' => their.get_counter_hand(),
            _ => unreachable!(),
        };

        points += get_points(&get_result(&our, &their));
        points += Hand::get_points(&our);
    }

    println!("Total score: {}", points);
}

#[derive(Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissor,
}

enum Result {
    Win,
    Draw,
    Loss,
}

impl Hand {
    fn get_type_part1(c: &char) -> Hand {
        match c {
            'A' | 'X' => Hand::Rock,
            'B' | 'Y' => Hand::Paper,
            'C' | 'Z' => Hand::Scissor,
            _ => unreachable!(),
        }
    }

    fn get_type_part2(c: &char) -> Hand {
        match c {
            'A' => Hand::Rock,
            'B' => Hand::Paper,
            'C' => Hand::Scissor,
            _ => unreachable!(),
        }
    }

    fn get_points(&self) -> usize {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissor => 3,
        }
    }

    fn get_counter_hand(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissor,
            Hand::Scissor => Hand::Rock,
        }
    }

    fn get_losing_hand(&self) -> Hand {
        self.get_counter_hand().get_counter_hand()
    }
}

/// True if our hand won.
fn get_result(our: &Hand, their: &Hand) -> Result {
    match (our, their) {
        (Hand::Rock, Hand::Paper) => Result::Loss,
        (Hand::Rock, Hand::Scissor) => Result::Win,
        (Hand::Paper, Hand::Rock) => Result::Win,
        (Hand::Paper, Hand::Scissor) => Result::Loss,
        (Hand::Scissor, Hand::Rock) => Result::Loss,
        (Hand::Scissor, Hand::Paper) => Result::Win,
        _ => Result::Draw,
    }
}

// Return number of points based on result.
fn get_points(result: &Result) -> usize {
    match result {
        Result::Win => 6,
        Result::Draw => 3,
        Result::Loss => 0,
    }
}
