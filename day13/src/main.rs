use std::cmp::Ordering;

use nom::{
    branch::alt,
    bytes::complete::{tag},
    character::complete::{char, digit1},
    combinator::{map_res},
    error::{ErrorKind},
    multi::separated_list0,
    sequence::{delimited},
    IResult,
};

fn main() {
    let input = include_str!("input.txt");
    let res = solve_part1(input);
    dbg!(res);
    let res = solve_part2(input);
    dbg!(res);
}

fn solve_part1(i: &str) -> usize {
    let inputs = i.split("\n\n");
    let mut res = 0;
    for (i, pair) in inputs.enumerate() {
        let pair: Vec<Packet> = pair
            .lines()
            .map(|i: &str| parse_packet(i).unwrap().1)
            .collect();

        if pair[0] < pair[1] {
            res += i + 1;
        }
    }
    res
}

fn solve_part2(i: &str) -> usize {
    let mut res = 1; 
    let inputs = i.split("\n\n");
    let mut packets: Vec<Packet> = vec![];

    for pair in inputs {
        let mut pair: Vec<Packet> = pair
            .lines()
            .map(|i: &str| parse_packet(i).unwrap().1)
            .collect();
        packets.push(pair.remove(0));
        packets.push(pair.remove(0));
    }

    let (a, b) = (parse_packet("[[2]]").unwrap().1, parse_packet("[[6]]").unwrap().1);
    packets.push(a);
    packets.push(b);

    let (a, b) = (parse_packet("[[2]]").unwrap().1, parse_packet("[[6]]").unwrap().1);
    packets.sort();

    for (i, elem) in packets.iter().enumerate() {
        if elem == &a || elem == &b {
            res *= i + 1;
        }
    }
    res
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Packet {
    data: Data,
}

#[derive(Debug, PartialEq, Eq)]
enum Data {
    Packets(Vec<Data>),
    Num(i64),
}

fn data_from(v: Vec<i64>) -> Data {
    let l = v.iter().map(|x| Data::Num(*x)).collect();
    Data::Packets(l)
}

impl Packet {
    fn default() -> Self {
        Packet {
            data: Data::Packets(vec![]),
        }
    }

    fn from(data: Data) -> Self {
        Packet { data }
    }
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Data::Num(a) => match other {
                Data::Num(b) => {
                    if a < b {
                        return Ordering::Less;
                    } else if a > b {
                        return Ordering::Greater;
                    }
                    Ordering::Equal
                }
                b => Data::Packets(vec![Data::Num(*a)]).cmp(b),
            },
            a => match other {
                Data::Num(b) => a.cmp(&Data::Packets(vec![Data::Num(*b)])),
                Data::Packets(b) => match a {
                    Data::Packets(a) => {
                        let (mut i, mut j) = (0, 0);
                        while i < a.len() && j < b.len() {
                            if a[i] == b[j] {
                                i += 1;
                                j += 1;
                            } else {
                                return a[i].cmp(&b[j]);
                            }
                        }
                        if i == a.len() {
                            Ordering::Less
                        } else {
                            Ordering::Greater
                        }
                    }
                    _ => Ordering::Less,
                },
            },
        }
    }
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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
    let (i, data) = delimited(
        char('['),
        separated_list0(tag(","), alt((parse_num, parse_list2))),
        char(']'),
    )(i)?;

    Ok((i, Data::Packets(data)))
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;
    static TEST_INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(13, solve_part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(140, solve_part2(TEST_INPUT));
    }

    #[test]
    fn test_parse_list() {
        let inputs = vec!["[1,1,3,1,1]", "[[1],4]"];
        let b = Data::Packets(vec![Data::Packets(vec![Data::Num(1)]), Data::Num(4)]);
        let expected = vec![
            Packet {
                data: data_from(vec![1, 1, 3, 1, 1]),
            },
            Packet { data: b },
        ];

        for (a, e) in inputs.iter().zip(expected) {
            assert_eq!(e, parse_packet(a).unwrap().1);
        }
    }

    #[test]
    fn test_cmp() {
        let p1 = "[[9,[3,[9,8,10,5],4,[8],[8,7,0,5]],6],[],[5]]";
        let p2 = "[[[[10,4,8],[1,6,8,3]]],[4,4,7],[[[9,2,6],[2,8],[5,9,0],2,0],5],[]]";
        let (a, b) = (parse_packet(p1).unwrap().1, parse_packet(p2).unwrap().1);
        assert_eq!(a.cmp(&b), Ordering::Less);
    }
}
