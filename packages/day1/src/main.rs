use std::fs;

fn main() {
    let data = fs::read_to_string("packages/day1/resources/input").expect("Failed to read file.");

    let mut elf_calories: Vec<usize> = data
        .split("\n\n")
        .map(|e| {
            e.split('\n')
                .map(|e| e.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .collect();

    // Part 1
    let mut v: Vec<usize> = Vec::new();
    v.push(500);
    println!(
        "The most calories carried by one elf is: {} [Calories]",
        elf_calories.iter().max().unwrap()
    );

    // Part 2
    let mut top_three: Vec<usize> = vec![0; 3];
    elf_calories.iter().for_each(|x| {
        let mut index = 0;

        for (i, val) in top_three.iter().enumerate() {
            if *val < top_three[index] {
                index = i;
            }
        }

        if *x > top_three[index] {
            top_three[index] = *x;
        }
    });
    println!(
        "Top three elves combined calories: {:?}",
        top_three.iter().sum::<usize>()
    );
}
