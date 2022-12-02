use std::fs;
use std::io::{BufRead, BufReader};

fn main() {
    let file = fs::File::open("packages/day2/resources/input_real").unwrap();
    let lines = BufReader::new(file).lines();
    let mut points: usize = 0;

    for line in lines {
        let line: Vec<char> = line.unwrap().chars().collect();

        let action = &line[2];
        let their = Hand::get_type(&line[0]);
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
    fn get_type(c: &char) -> Hand {
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
