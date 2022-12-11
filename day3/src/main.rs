use std::collections::HashSet;

fn main() {
    let puzz_input = include_str!("input.txt");
    let result = puzz_input.lines().fold(0, |x, w| x + check_backpack(w));
    println!("The result is {}", result);
    let puzz_lines: Vec<&str> = puzz_input.lines().collect();
    let mut i = 0;
    let mut res2 = 0;
    // This is hacky and not very rust-esque
    while i < puzz_lines.len() {
        res2 += group_check(puzz_lines[i], puzz_lines[i + 1], puzz_lines[i + 2]);
        i += 3;
    }
    println!("The group result is {}", res2);
}

fn group_check(a: &str, b: &str, c: &str) -> u32 {
    // A lot of var initialisation going on that I'd rather skip but it works
    let a_s: HashSet<char> = a.chars().collect();
    let b_s: HashSet<char> = b.chars().filter(|x| a_s.contains(x)).collect();
    for ch in c.chars() {
        if b_s.contains(&ch) {
            return priority(ch);
        }
    }
    0
}

fn check_backpack(backpack: &str) -> u32 {
    let mid = backpack.len() / 2;
    let (a, b) = backpack.split_at(mid);
    let first: HashSet<char> = a.chars().collect();
    for c in b.chars() {
        if first.contains(&c) {
            return priority(c);
        }
    }
    0
}

fn priority(c: char) -> u32 {
    let val = c as u32;

    // println!("{} has priority {}", c, res);
    match val <= ('Z' as u32) {
        true => val - 'A' as u32 + 27,
        false => val - 'a' as u32 + 1,
    }
}
