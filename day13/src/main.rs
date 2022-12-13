use std::io::BufRead;

use nom::{IResult, multi::separated_list0, branch::alt, character::complete::{char, digit0, digit1}, bytes::complete::{tag, take_until, take_while}, sequence::{tuple, delimited}, combinator::{opt, map_res}, error::{ParseError, ErrorKind}};

fn main() {
    let input = include_str!("input.txt");
    let res = solve_part1(input);
    dbg!(res);
}

fn solve_part1(i: &str) -> usize {
    let inputs = i.split("\n\n");
    let mut res = 0;
    for (i, pair) in inputs.enumerate() {
        let pair: Vec<Packet> = pair.lines().map(|i: &str| parse_packet(i).unwrap().1).collect();
        if pair[0] < pair[1] {
            dbg!(i);
            res += i + 1;
        }
    }
    res
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Packet {
    data: Data
} 

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Data {
    Packets(Vec<Data>),
    Num(i64)
}

fn data_from(v: Vec<i64>) -> Data {
    let l = v.iter().map(|x| Data::Num(*x)).collect();
    Data::Packets(l)
}

impl Packet {
    fn default() -> Self {
        Packet {
            data: Data::Packets(vec![])
        }
    }

    fn from(data: Data) -> Self {
        Packet {
            data: data
        }
    }
}

fn compare_list(a: &[Data], b: &[Data]) -> bool {
    false
}

fn parse_packet(i: &str) -> IResult<&str, Packet> {
    let (i, data) = parse_list2(i)?;
    
    Ok((i, Packet::from(data)))
}

fn parse_num(i: &str) -> IResult<&str, Data> {
    map_res(digit1, |s: &str| {
        Ok::<Data, ErrorKind>(Data::Num(s.parse::<i64>().unwrap()))
    })(i)
}

fn parse_list2(i: &str) -> IResult<&str, Data> {
    let (i, data) = delimited(char('['), separated_list0(tag(","), alt((parse_num, parse_list2))), char(']'))(i)?;

    Ok((i, Data::Packets(data)))
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;
    static TEST_INPUT: &str = include_str!("test-input.txt"); 

    #[test]
    fn test_part1() {
        assert_eq!(14, solve_part1(TEST_INPUT)); 
    }

    #[test]
    fn test_parse_list() {
        let inputs = vec![
            "[1,1,3,1,1]",
            "[[1],4]"
        ];
        let b = Data::Packets(vec![ Data::Packets(vec![Data::Num(1)]), Data::Num(4)]);
        let expected = vec![
            Packet{data: data_from(vec![1,1,3,1,1])},
            Packet{data: b}
        ];

        for (a, e) in inputs.iter().zip(expected) {
            assert_eq!(e, parse_packet(a).unwrap().1);
        }
    }
}
