use nom::{IResult, bytes::complete::{tag, take_till, take_while, take_until}, branch::alt, combinator::{map_res, opt}, sequence::tuple, character::complete::digit1, multi::separated_list0};

fn main() {
    println!("Hello, world!");

}

#[derive(Debug, PartialEq, Eq)]
struct Monkey {
    items: Vec<i64>,
    operation: fn(i64) -> i64,
    division_test: i64, 
    next_monkey: (usize, usize)
}


fn parse_number(i: &str) -> IResult<&str, i64> {
    map_res(take_while(|c: char| c.is_ascii_digit()), |s: &str| s.parse::<i64>())(i)
}

fn parse_number_list(i: &str) -> IResult<&str, Vec<i64>> {
    separated_list0(tag(", "), parse_number)(i)
}

fn parse_equation(i: &str) -> IResult<&str, fn(i64) -> i64> {
    // parse symbol + or *, and then i64 or old
}

fn parse_monkey(i: &str) -> IResult<&str, Monkey> {
    let (i, _) = take_until("\n")(i)?; // Get rid of first line
    let (i, _) = take_while(|c: char| !c.is_ascii_digit())(i)?;
    let (i, nums) = parse_number_list(i)?; 
    let (i, _) = take_until("=")(i)?;
    dbg!(i);


    let monkey = Monkey {
        items: nums,
        operation: |a| a * 19,
        division_test: 23,
        next_monkey: (2, 3)
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
    use std::vec;
    use super::*;
    static TEST_INPUT: &str = include_str!("test-input.txt"); 

    #[test]
    fn test_monkey_parse() {
        let monkey_str = TEST_INPUT.split_once("\n\n").unwrap().0;
        let expected = Monkey {
            items: vec![79, 89],
            operation: |a| a * 19,
            division_test: 23,
            next_monkey: (2, 3)
        };
        assert_eq!(parse_monkey(monkey_str).unwrap().1, expected);
    }

    #[test]
    fn test_number_parse() {
        assert_eq!(parse_number("123").unwrap().1, 123); 
    }
}
