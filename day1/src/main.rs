use std::env;
use std::{cmp::max, fs};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let binding = fs::read_to_string("input.txt").unwrap();
    let elf_contents = binding.lines();
    let mut puzzle_input: Vec<Vec<i32>> = Vec::new();
    let mut curr_elf = 0;
    puzzle_input.push(Vec::new());
    for line in elf_contents {
        if line.eq("") {
            puzzle_input.push(Vec::new());
            curr_elf += 1;
        } else {
            match line.parse::<i32>() {
                Ok(n) => puzzle_input.get_mut(curr_elf).unwrap().push(n),
                _ => (),
            }
        }
    }
    let result_one = most_calories(&puzzle_input);
    println!("The elf with the most calories has {result_one}.");
    let result_two = top_three_cals_total(&puzzle_input);
    println!("The three top elves have {result_two}.");
}

fn most_calories(elves: &Vec<Vec<i32>>) -> i32 {
    println!("There are {} elves", elves.len());
    let mut max_cals = 0;
    for elf in elves {
        let mut cals = 0;
        for snack in elf {
            cals += snack;
        }
        max_cals = max(max_cals, cals);
    }
    max_cals
}

fn top_three_cals_total(elves: &Vec<Vec<i32>>) -> i32 {
    let mut elf_cals = vec![0; elves.len()];
    let mut curr_elf = 0;
    for elf in elves {
        elf_cals[curr_elf] = elf.iter().sum();
        curr_elf += 1;
    }
    elf_cals.sort_by(|a, b| b.cmp(a));
    let mut result = 0;
    for i in 0..3 {
        result += elf_cals[i];
    }
    result
}
