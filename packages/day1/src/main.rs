use std::fs;

fn main() {
    let data =
        fs::read_to_string("packages/day1/resources/input_real").expect("Failed to read file.");

    let elf_calories = data.split("\n\n").map(|e| {
        e.split('\n')
            .map(|e| e.parse::<usize>().unwrap())
            .sum::<usize>()
    });

    println!(
        "The most calories carried by one elf is: {} [Calories]",
        elf_calories.max().unwrap()
    );
}
