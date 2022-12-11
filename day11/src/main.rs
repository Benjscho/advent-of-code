use std::collections::VecDeque;

use nom::{
    bytes::complete::{tag, take, take_until, take_while},
    combinator::{map_res},
    multi::separated_list0,
    sequence::tuple,
    IResult,
};

fn main() {
    let input = include_str!("input.txt");
    let res = solve_part1(input);
    dbg!(res);
    let res = solve_part2(input);
    dbg!(res);
}

#[derive(Debug, PartialEq, Eq)]
struct Monkey {
    items: VecDeque<i64>,
    operation: (char, Option<i64>),
    division_test: i64,
    next_monkey: (usize, usize),
    item_checks: i64,
}

fn solve_part1(i: &str) -> i64 {
    let mut monkeys: Vec<Monkey> = i
        .split("\n\n")
        .map(|s: &str| parse_monkey(s).unwrap().1)
        .collect();

    for _ in 0..20 {
        one_round(&mut monkeys);
    }

    let mut checks: Vec<i64> = monkeys.iter().map(|m: &Monkey| m.item_checks).collect();
    checks.sort_by(|a, b| b.cmp(a));
    checks[0] * checks[1]
}

fn solve_part2(i: &str) -> i64 {
    let mut monkeys: Vec<Monkey> = i
        .split("\n\n")
        .map(|s: &str| parse_monkey(s).unwrap().1)
        .collect();

    let common_divisor: i64 = monkeys.iter().fold(1, |a, m| a * m.division_test);

    for _ in 0..10000 {
        one_round_part2(&mut monkeys, common_divisor);
    }

    let mut checks: Vec<i64> = monkeys.iter().map(|m: &Monkey| m.item_checks).collect();
    checks.sort_by(|a, b| b.cmp(a));
    checks[0] * checks[1]
}

fn one_round(monkeys: &mut Vec<Monkey>) {
    for i in 0..monkeys.len() {
        while !monkeys[i].items.is_empty() {
            monkeys[i].item_checks += 1;
            let item = monkeys[i].items.pop_front();
            let nw = next_worry_level(item.unwrap(), monkeys[i].operation) / 3;
            let next_monkey = monkeys[i].next_monkey;
            if nw % monkeys[i].division_test == 0 {
                monkeys[next_monkey.0].items.push_back(nw);
            } else {
                monkeys[next_monkey.1].items.push_back(nw);
            }
        }
    }
}

fn one_round_part2(monkeys: &mut Vec<Monkey>, common_worry: i64) {
    for i in 0..monkeys.len() {
        while !monkeys[i].items.is_empty() {
            monkeys[i].item_checks += 1;
            let item = monkeys[i].items.pop_front();
            let nw = next_worry_level(item.unwrap(), monkeys[i].operation) % common_worry;
            let next_monkey = monkeys[i].next_monkey;
            if nw % monkeys[i].division_test == 0 {
                monkeys[next_monkey.0].items.push_back(nw);
            } else {
                monkeys[next_monkey.1].items.push_back(nw);
            }
        }
    }
}

fn next_worry_level(worry: i64, op: (char, Option<i64>)) -> i64 {
    match op.0 {
        '+' => match op.1 {
            Some(x) => worry + x,
            _ => worry + worry,
        },
        '*' => match op.1 {
            Some(x) => worry * x,
            _ => worry * worry,
        },
        _ => worry,
    }
}

fn parse_number(i: &str) -> IResult<&str, i64> {
    map_res(take_while(|c: char| c.is_ascii_digit()), |s: &str| {
        s.parse::<i64>()
    })(i)
}

fn parse_number_list(i: &str) -> IResult<&str, Vec<i64>> {
    separated_list0(tag(", "), parse_number)(i)
}

/// I was going to originally pass this into a closure, but then I found out
/// about how you can't capture a variable in a closure. Figured it makes
/// more sense to parse this into a tuple here and do the resulting
/// matching in the logic of next_worry_level
fn parse_equation(i: &str) -> IResult<&str, (char, Option<i64>)> {
    let (i, _) = tag("= old ")(i)?;
    let (i, (op, _, num)) = tuple((take(1_usize), tag(" "), take_until("\n")))(i)?;
    let op = op.chars().next().unwrap();
    match num {
        j if j.chars().next().unwrap().is_ascii_digit() => {
            Ok((i, (op, Some(j.parse::<i64>().unwrap()))))
        }
        _ => Ok((i, (op, None))),
    }
}

fn parse_monkey(i: &str) -> IResult<&str, Monkey> {
    let (i, _) = take_until("\n")(i)?; // Get rid of first line
    let (i, _) = take_while(|c: char| !c.is_ascii_digit())(i)?;
    let (i, nums) = parse_number_list(i)?;
    let (i, _) = take_until("=")(i)?;
    let (i, operation) = parse_equation(i)?;
    let (i, (_, div)) = tuple((tag("\n  Test: divisible by "), parse_number))(i)?;
    let (i, (_, m1)) = tuple((tag("\n    If true: throw to monkey "), parse_number))(i)?;
    let (i, (_, m2)) = tuple((tag("\n    If false: throw to monkey "), parse_number))(i)?;

    let nums = VecDeque::from(nums);

    let monkey = Monkey {
        items: nums,
        operation,
        division_test: div,
        next_monkey: (m1 as usize, m2 as usize),
        item_checks: 0,
    };
    Ok((i, monkey))
}

// Parse the input
// - Split on \n\n
//
// Parse monkey:
// - item numbers
// - Operation
// - Item transition logic

#[cfg(test)]
mod test {
    use super::*;
    use std::vec;
    static TEST_INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_solve_part1() {
        assert_eq!(10605, solve_part1(TEST_INPUT));
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(2713310158, solve_part2(TEST_INPUT));
    }

    #[test]
    fn test_monkey_parse() {
        let monkey_str = TEST_INPUT.split_once("\n\n").unwrap().0;
        let expected = Monkey {
            items: VecDeque::from(vec![79, 98]),
            operation: ('*', Some(19)),
            division_test: 23,
            next_monkey: (2, 3),
            item_checks: 0,
        };
        assert_eq!(parse_monkey(monkey_str).unwrap().1, expected);
    }

    #[test]
    fn test_number_parse() {
        assert_eq!(parse_number("123").unwrap().1, 123);
    }

    #[test]
    fn test_equation_parse() {
        let a = vec!["= old * 19\n", "= old * old\n", "= old + 17\n"];
        let b = vec![('*', Some(19_i64)), ('*', None), ('+', Some(17))];

        for (e, actual) in a.iter().zip(b) {
            assert_eq!(actual, parse_equation(e).unwrap().1);
        }
    }
}
