use nom::{IResult, multi::separated_list0, character::complete::digit0, bytes::complete::tag};

fn main() {
    println!("Hello, world!");
}

fn solve_part1(i: &str) -> i64 {
    unimplemented!()
}

#[derive(Debug, PartialEq, Eq)]
struct Packet {
    data: Data
} 

#[derive(Debug, PartialEq, Eq)]
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
            data: Data::Packets(Vec::new())
        }
    }

    fn from(data: Data) -> Self {
        Packet {
            data
        }
    }
}

fn compare_list(a: &[Data], b: &[Data]) -> bool {
    if a.is_empty() {
        return true;
    } 
    let (x, y) = (a.first(), b.first());
    match y {
        None => return false,
        Some(Data::Num(n)) => {

        },
        Some(Data::Packets(n)) => {

        }
    }
    
    false
}

fn parse_packet(i: &str) -> IResult<&str, Packet> {
    let (i, _) = tag("[")(i)?;
    let (i, data) = parse_list(i)?;
    
    Ok((i, Packet::from(data)))
}

fn parse_list(i: &str) -> IResult<&str, Data> {
    let mut p = vec![];
    let (i, nums) = separated_list0(tag(","), digit0)(i)?;
    for x in &nums {
        p.push(Data::Num(x.parse::<i64>().unwrap()));
    }

    match i.chars().next() {
        Some('[') => {
            let (i, d) = parse_list(i)?;
            p.push(d);
        }
        _ => ()
    }
    dbg!(i, nums);

    Ok((i, Data::Packets(p)))
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
    fn test_parse_list() {
        let inputs = vec![
            "[1,1,3,1,1]",
            "[[1],4]"
        ];
        let ex2 = vec![];
        ex2.push(Data::Num(1));
        let ex = vec![];
        ex.push(ex2);
        ex.push(Data::Num(4));
        let expected = vec![
            Packet{data: data_from(vec![1,1,3,1,1])},
            Packet{data: vec![
                ex2,
            ]}
        ];


        for (a, e) in inputs.iter().zip(expected) {
            assert_eq!(e, parse_packet(a).unwrap().1);
        }
    }
}
