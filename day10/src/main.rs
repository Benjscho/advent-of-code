use std::collections::VecDeque;

fn main() {
    let input = include_str!("input.txt");
    let res = solve_part1(input);
    dbg!(res);
    let screen_display = solve_part2(input);

    println!("{}", screen_display);
}

fn solve_part1(input: &str) -> i64 {
    let mut res = 0;
    let mut ops: VecDeque<Option<i64>> = VecDeque::new();

    for l in input.lines() {
        let s = l.split_once(' ');

        match s {
            Some((_, b)) => ops.push_back(Some(b.parse::<i64>().unwrap())),
            _ => ops.push_back(None),
        }
    }

    let impt_cycles = vec![20, 60, 100, 140, 180, 220];

    let mut x = 1;
    let mut temp: Option<i64> = None;
    for i in 1..=220 {
        if impt_cycles.contains(&i) {
            res += i * x;
        }
        match temp {
            Some(val) => {
                x += val;
                temp = None;
            }
            None => {
                let instruction = ops.pop_front();
                match instruction {
                    Some(x) => temp = x,
                    _ => (),
                }
            }
        }
    }

    res
}

fn solve_part2(input: &str) -> String {
    let mut ops: VecDeque<Option<i64>> = VecDeque::new();

    for l in input.lines() {
        let s = l.split_once(' ');

        match s {
            Some((_, b)) => ops.push_back(Some(b.parse::<i64>().unwrap())),
            _ => ops.push_back(None),
        }
    }

    let mut res_vec: Vec<char> = vec![];
    let mut x = 1;
    let mut temp: Option<i64> = None;
    for i in 0..240 {
        let pixel = i % 40;
        if pixel == 0 && i != 0 {
            res_vec.push('\n');
        }
        if x - 1 == pixel || x == pixel || x + 1 == pixel {
            res_vec.push('#');
        } else {
            res_vec.push('.');
        }
        match temp {
            Some(val) => {
                x += val;
                temp = None;
            }
            None => {
                let instruction = ops.pop_front();
                match instruction {
                    Some(x) => temp = x,
                    _ => (),
                }
            }
        }
    }
    res_vec.push('\n');

    res_vec.iter().cloned().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    static TEST_INPUT: &str = include_str!("test-input.txt");
    static TEST_OUTPUT: &str = include_str!("test-output.txt");

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 13140);
    }

    #[test]
    fn test_part2() {
        env::set_var("RUST_BACKTRACE", "1");
        assert_eq!(solve_part2(TEST_INPUT), TEST_OUTPUT);
    }
}
