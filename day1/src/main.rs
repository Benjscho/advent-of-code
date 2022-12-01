use std::{cmp::max, fs}; 
use std::env;

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
                _ => ()
            }
        } 
    }
    let result = most_calories(puzzle_input); 
    println!("The answer is {result}."); 
}

fn most_calories(elves: Vec<Vec<i32>>) -> i32 {
    println!("There are {} elves", elves.len()); 
    let mut max_cals = 0; 
    for elf in elves {
        let mut cals = 0; 
        for snack in elf {
            cals = cals + snack;
        }
        max_cals = max(max_cals, cals);
    }
    max_cals
}
