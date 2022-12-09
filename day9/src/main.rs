use std::env;
use std::cmp::{max, min};
use core::fmt;
use std::collections::HashSet;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    println!("Hello, world!");
    let input = include_str!("input.txt"); 
    let res = solve_part1(input); 
    dbg!(res);

    let res = solve_part2(input);
    dbg!(res);
}

// Head and tail exist as two points on an infinite grid
// Visited is a hashset of grid points that tail has visited

fn solve_part1(input: &str) -> usize {
    let mut tail_visited: HashSet<Point> = HashSet::new();
    let mut state = Link::default();
    for line in input.lines() {
        let (dir, n) = line.split_once(" ").unwrap();
        let n = n.parse::<usize>().unwrap();

        for _ in 0..n {
            state.next_state(dir);
            tail_visited.insert(state.tail.clone());
        }
    }
    tail_visited.len()
}

fn solve_part2(input: &str) -> usize {
    let mut tail_visited: HashSet<Point> = HashSet::new();
    let mut rope = Rope::new();
    for line in input.lines() {
        let (dir, n) = line.split_once(" ").unwrap();
        let n = n.parse::<usize>().unwrap();

        for _ in 0..n {
            rope.move_rope(dir);
            tail_visited.insert(rope.links[9].clone());
        }
    }
    tail_visited.len()
}


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32, 
    y: i32
}

#[derive(Debug, Clone, Copy)]
struct Link {
    head: Point,
    tail: Point
}

struct Rope {
    links: [Point; 10]
}

impl Rope {
    fn new() -> Self {
        Rope {
            links: [Point {x: 0, y: 0}; 10]
        }
    }

    fn move_rope(&mut self, dir: &str) {
        match dir {
            "R" => self.links[0].x += 1,
            "L" => self.links[0].x -= 1,
            "U" => self.links[0].y += 1,
            "D" => self.links[0].y -= 1,
            _ => {}
        }
        self.chase_points();
    }

    fn chase_points(&mut self) {
        for i in 1..self.links.len() {
            let dx = self.links[i - 1].x as i32 - self.links[i].x as i32;
            let dy = self.links[i - 1].y as i32 - self.links[i].y as i32;

            match dx {
                x if x > 1 => {
                    self.links[i].x += 1;
                    match dy {
                        x if x >= 1 => self.links[i].y += 1,
                        x if x <= -1 => self.links[i].y -= 1,
                        _ => {}
                    }
                }, 
                x if x < -1 => {
                    self.links[i].x -= 1;
                    match dy {
                        x if x >= 1 => self.links[i].y += 1,
                        x if x <= -1 => self.links[i].y -= 1,
                        _ => {}
                    }
                },
                _ => {
                    match dy {
                        x if x > 1 => {
                            self.links[i].y += 1;
                            match dx {
                                x if x >= 1 => self.links[i].x += 1,
                                x if x <= -1 => self.links[i].x -= 1,
                                _ => {}
                            }
                        }, 
                        x if x < -1 => {
                            self.links[i].y -= 1;
                            match dx {
                                x if x >= 1 => self.links[i].x += 1,
                                x if x <= -1 => self.links[i].x -= 1,
                                _ => {}
                            }
                        },
                        _ => {}
                    }
                }
            }
            
        }

    }
}

impl fmt::Debug for Rope {
    // This is absolutely disgusting code, sorry.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let min_x = self.links.map(|l| l.x).iter().min().unwrap().to_owned();
        let max_x = self.links.map(|l| l.x).iter().max().unwrap().to_owned();
        let min_y = self.links.map(|l| l.y).iter().min().unwrap().to_owned();
        let max_y = self.links.map(|l| l.y).iter().max().unwrap().to_owned();
        let x = (max_x - min_x +1)  as usize;
        let y = (max_y - min_y +1) as usize;
        let mut grid = vec![vec![".".to_string();x];y];
        for (i, link) in self.links.iter().enumerate() {
            let curr_y = (link.y - min_y);
            println!("{}", curr_y);
            grid.get_mut((link.y - min_y) as usize).unwrap()[(link.x - min_x) as usize] = i.to_string();
        }
        writeln!(f, "");

        for line in grid.iter().rev() {
            for x in line.iter() {
                write!(f, "{}", x);
            }
            writeln!(f, "");
        }
        Ok(())
    }
}

impl Link {
    fn default() -> Self {
        Link {
            head: Point {x: 0, y: 0},
            tail: Point {x: 0, y: 0},
        }
    }

    fn next_state(&mut self, dir: &str) {
        match dir {
            "R" => self.head.x += 1,
            "L" => self.head.x -= 1,
            "U" => self.head.y += 1,
            "D" => self.head.y -= 1,
            _ => {}
        }
        self.chase_head();
    }

    fn chase_head(&mut self) {
        let dx = self.head.x as i32 - self.tail.x as i32;
        let dy = self.head.y as i32 - self.tail.y as i32;

        match dx {
            i if i > 1 => {
                self.tail.x += 1;
                match dy {
                    i if i >= 1 => self.tail.y += 1,
                    i if i <= -1 => self.tail.y -= 1,
                    _ => {}
                }
            }, 
            i if i < -1 => {
                self.tail.x -= 1;
                match dy {
                    i if i >= 1 => self.tail.y += 1,
                    i if i <= -1 => self.tail.y -= 1,
                    _ => {}
                }
            },
            _ => {
                match dy {
                    i if i > 1 => {
                        self.tail.y += 1;
                        match dx {
                            i if i >= 1 => self.tail.x += 1,
                            i if i <= -1 => self.tail.x -= 1,
                            _ => {}
                        }
                    }, 
                    i if i < -1 => {
                        self.tail.y -= 1;
                        match dx {
                            i if i >= 1 => self.tail.x += 1,
                            i if i <= -1 => self.tail.x -= 1,
                            _ => {}
                        }
                    },
                    _ => {}
                }
            }
        }
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    static TEST_INPUT: &str = include_str!("test-input.txt");
    static TEST_INPUT2: &str = include_str!("test-input2.txt");

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 13);
    }

    #[test]
    fn test_part2() {
        println!("SOLVING FIRST TEST INPUT");
        solve_part2(TEST_INPUT);
        println!("SOLVING SECOND TEST INPUT");
        assert_eq!(solve_part2(TEST_INPUT2), 36);
    }
}
