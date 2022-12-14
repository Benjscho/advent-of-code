use std::cmp::{min, max};
use nom::{
    bytes::complete::tag, character::complete::digit1, multi::separated_list1, sequence::tuple,
    IResult,
};

fn main() {
    let input = include_str!("input.txt");
    let res = solve_part1(input);
    dbg!(res);
    let res = solve_part2(input);
    dbg!(res);
}

fn solve_part1(i: &str) -> i64 {
    let mut c = Cave::new();
    let mut x = 0;
    for line in i.lines() {
        let l = parse_line(line);
        c.apply_line(l);
    }

    while c.drop_sand(false) {
        x += 1;
    }

    x
}

fn solve_part2(i: &str) -> i64 {
    let mut c = Cave::new();
    let mut x = 0;
    for line in i.lines() {
        let l = parse_line(line);
        c.apply_line(l);
    }
    c.add_floor();

    while c.drop_sand(true) {
        x += 1;
    }

    x
}

struct Cave {
    grid: [bool; 500_000],
    highest: usize,
}

impl Cave {
    fn new() -> Self {
        Cave {
            grid: [false;500_000],
            highest: 0,
        }
    }

    fn get_space(&self, x: usize, y: usize) -> bool {
        self.grid[y * 1000 + x]
    }

    fn set_space(&mut self, x: usize, y: usize) {
        self.grid[y * 1000 + x] = true;
    }

    fn apply_line(&mut self, mut line: Line) {
        let (mut cx, mut cy) = line.points.pop().unwrap();
        while !line.points.is_empty() {
            let (nx, ny) = line.points.pop().unwrap();
            if cx == nx {
                let (from, to) = (min(cy, ny), max(cy, ny));
                for i in from..=to {
                    self.set_space(cx, i);
                }
            } else {
                let (from, to) = (min(cx, nx), max(cx, nx));
                for i in from..=to {
                    self.set_space(i, cy);
                }
            }

            if cy > self.highest {
                self.highest = cy
            }
            if ny > self.highest {
                self.highest = ny
            }

            (cx, cy) = (nx, ny);
        }
    }

    fn add_floor(&mut self) {
        for i in 0..1000 {
            self.set_space(i, self.highest + 2);
        }
    }

    /// This function drops sand in at (500, 0) where it ascends until
    /// it either comes to rest or falls off the edge. If it comes to rest
    /// we return `true`.
    fn drop_sand(&mut self, part2: bool) -> bool {
        let (mut cx, mut cy) = (500, 0);
        if self.get_space(500, 0) {
            return false;
        }
        
        while cy < self.highest || part2 {
            let (a, b, c) = (self.get_space(cx-1, cy+1), self.get_space(cx, cy+1), self.get_space(cx + 1, cy + 1));
            match b {
                false => {cy = cy + 1},
                true => {
                    match a {
                        false => {
                            cy += 1;
                            cx -= 1;
                        },
                        true => {
                            match c {
                                false => {
                                    cy += 1;
                                    cx += 1;
                                },
                                _ => {
                                    self.set_space(cx, cy);
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
        false
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Line {
    points: Vec<(usize, usize)>,
}

fn parse_line(i: &str) -> Line {
    let (_i, points) = if let Ok((i, points)) = separated_list1(tag(" -> "), parse_point)(i) {
        (i, points)
    } else {
        todo!()
    };
    Line { points }
}

fn parse_point(i: &str) -> IResult<&str, (usize, usize)> {
    let (i, (a, _, b)) = tuple((digit1, tag(","), digit1))(i)?;
    Ok((i, (a.parse().unwrap(), b.parse().unwrap())))
}

#[cfg(test)]
mod test {
    use super::*;
    static TEST_INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(24, solve_part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(93, solve_part2(TEST_INPUT));
    }

    #[test]
    fn test_line_parse() {
        let s = "498,4 -> 498,6 -> 496,6";
        let expected = Line {
            points: vec![(498, 4), (498, 6), (496, 6)],
        };
        assert_eq!(expected, parse_line(s));
    }

    #[test]
    fn apply_line() {
        let mut g = Cave::new();
        assert_eq!(false, g.get_space(498, 4));
        let line = Line {
            points: vec![(498, 4), (498, 6), (496, 6)],
        };
        g.apply_line(line);
        assert_eq!(true, g.get_space(498, 5));
    }
}
