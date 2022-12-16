use std::{collections::HashSet, cmp::min};

use nom::{IResult, bytes::complete::{tag, take_until1, take_while}, sequence::tuple};

fn main() {
    let input = include_str!("input.txt");
    let res = solve_part1(input, 2000000);
    dbg!(res);
    let res = solve_part2(input);
    dbg!(res);
}

fn solve_part1(i: &str, row: i64) -> usize {
    let mut sensors: Vec<Sensor> = vec![];
    let mut beacons: HashSet<Position> = HashSet::new();

    for l in i.lines() {
        let (s, b) = parse_line(l).unwrap().1;
        sensors.push(s);
        beacons.insert(b);
    }
    let mut coords: HashSet<Position> = HashSet::new();

    for s in sensors {
        let pos = s.position;
        let dist = (row - pos.1).abs();
        let mut i = 0;
        while dist + i <= s.beacon_radius {
            let (a, b) = ((pos.0 - i, row), (pos.0 + i, row));
            coords.insert(a);
            coords.insert(b);
            i += 1;
        }
    }
    for pos in beacons {
        coords.remove(&pos);
    }

    coords.len() 
}

fn solve_part2(i: &str) -> i64 {

    let mut sensors: Vec<Sensor> = vec![];

    for l in i.lines() {
        let (s, _) = parse_line(l).unwrap().1;
        sensors.push(s);
    }

    let mut pos_lines = vec![];
    let mut neg_lines = vec![];

    for s in sensors {
        let (x, y) = s.position;
        pos_lines.push(x - y + s.beacon_radius);
        pos_lines.push(x - y - s.beacon_radius);
        neg_lines.push(x + y + s.beacon_radius);
        neg_lines.push(x + y - s.beacon_radius);
    }

    let mut pos_intercept = 0; 
    let mut neg_intercept = 0; 

    for i in 0..pos_lines.len() - 1 {
        for j in i+1..pos_lines.len() {
            if (pos_lines[i] - pos_lines[j]).abs() == 2 {
                pos_intercept = min(pos_lines[i], pos_lines[j]) + 1;
            }
            if (neg_lines[i] - neg_lines[j]).abs() == 2 {
                neg_intercept = min(neg_lines[i], neg_lines[j]) + 1;
            }
        }
    }

    let x = (pos_intercept + neg_intercept) / 2;
    let y = (neg_intercept - pos_intercept) / 2;

    (x * 4_000_000) + y
}

fn parse_coords(i: &str) -> IResult<&str, (i64, i64)> {
    let (i, (_, x, _, y)) = tuple((
        tag("x="),
        take_until1(","),
        tag(", y="),
        // Take while we haven't reached the next delim and
        // there's still coord chars
        take_while(|c: char| c.is_ascii() && c != ':')
    ))(i)?;

    Ok((i, (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap())))
}

fn parse_line(i: &str) -> IResult<&str, (Sensor, Position)> {
    let (i, (_, s, _, b)) = tuple((
        tag("Sensor at "),
        parse_coords,
        tag(": closest beacon is at "),
        parse_coords
    ))(i)?;

    let sensor = Sensor {position: s, beacon_radius: manhattan_distance(s, b)};

    Ok((i, (sensor, b)))
}

fn manhattan_distance(a: (i64, i64), b: (i64, i64)) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

#[derive(Debug, PartialEq, Eq)]
struct Sensor {
    position: Position,
    beacon_radius: i64
}

type Position = (i64, i64);

// Cave is large and sparse, so representing
// with a sparse matrix, list of sensors
// Should add beacons? Maybe a vec of tuples
// Where the third represents the type
//
// Near neighbour should be efficient! We check if each sensor's radius
// lies within that row.


#[cfg(test)]
mod test {
    use super::*;
    static TEST_INPUT: &str = include_str!("test-input.txt"); 

    #[test]
    fn test_part1() {
        assert_eq!(26, solve_part1(TEST_INPUT, 10)); 
    }

    #[test]
    fn test_parse_coords() {
        let i = "x=-15, y=14:";
        assert_eq!((-15, 14), parse_coords(i).unwrap().1);
    }

    #[test]
    fn test_parse_line() {
        let i = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15";
        let expected_b = (-2, 15);
        let expected_s = Sensor {position: (2, 18), beacon_radius: manhattan_distance((2, 18), (-2, 15))};
        assert_eq!((expected_s, expected_b), parse_line(i).unwrap().1);
    }
}
