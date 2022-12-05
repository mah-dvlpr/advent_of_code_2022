use std::fs;

type Layout = Vec<Vec<char>>;

fn main() {
    let input = fs::read_to_string("packages/day5/resources/input").unwrap();
    let data: Vec<&str> = input.split("\n\n").collect();

    // Part 1
    let layout_str = data[0];
    let procedure = data[1];
    let mut layout = init_layout(layout_str);
    run_procedure(&mut layout, &procedure, false);
    print_top_crates(&layout);

    // Part 2
    let mut layout = init_layout(layout_str);
    run_procedure(&mut layout, &procedure, true);
    print_top_crates(&layout);
}

fn print_top_crates(layout: &Layout) {
    print!("The code is: ");
    for stack in layout {
        print!("{}", stack.last().unwrap());
    }
    println!();
}

fn init_layout(layout_str: &str) -> Layout {
    const CRATE_WIDTH: usize = 3;
    const CRATE_SPACING: usize = 1;

    fn find_next_crate(crate_line: &Vec<char>, column: &mut usize) -> Option<char> {
        let mut index = *column * (CRATE_WIDTH + CRATE_SPACING);

        loop {
            if crate_line.len() <= index {
                return None;
            }

            if crate_line[index] != '[' {
                index += CRATE_WIDTH + CRATE_SPACING;
            } else {
                break;
            }
        }

        *column = index / (CRATE_WIDTH + CRATE_SPACING);
        Some(crate_line[index + 1])
    }

    let columns = layout_str
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let mut layout: Layout = Vec::new();

    for i in 0..columns {
        layout.insert(i, <Vec<char>>::new());
    }

    let mut column = 0;

    for line in layout_str.lines().rev().skip(1) {
        let crate_line = line.chars().collect::<Vec<char>>();

        while let Some(c) = find_next_crate(&crate_line, &mut column) {
            layout[column].push(c);
            column += 1;
        }

        column = 0;
    }

    layout
}

fn run_procedure(layout: &mut Layout, procedure: &str, keep_order: bool) {
    for step in procedure.lines() {
        let parts_str: Vec<&str> = step.split_whitespace().collect();
        move_crates(
            layout,
            parts_str[3].parse::<usize>().unwrap(),
            parts_str[5].parse::<usize>().unwrap(),
            parts_str[1].parse::<usize>().unwrap(),
            keep_order,
        );
    }
}

fn move_crates(layout: &mut Layout, from: usize, to: usize, amount: usize, keep_order: bool) {
    if keep_order {
        let len = layout[from - 1].len() - amount;
        let mut current_crates: Vec<char> = layout[from - 1].split_off(len);
        layout[to - 1].append(&mut current_crates);
    } else {
        for _ in 0..amount {
            let current_crate = layout[from - 1].pop().unwrap();
            layout[to - 1].push(current_crate);
        }
    }
}
