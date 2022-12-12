use std::collections::{HashSet, VecDeque};

fn main() {
    println!("Hello, world!");
    let input = include_str!("input.txt");
    let res = solve_part1(input); 
    dbg!(res);
    let res = solve_part2(input); 
    dbg!(res);
}

fn solve_part1(i: &str) -> i64 {
    let grid = Grid::from(i);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: VecDeque<(usize, usize, i64)> = VecDeque::new();
    queue.push_back((grid.s.0, grid.s.1, 0));
    visited.insert(grid.s);
    let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    while !queue.is_empty() {
        let (y, x, steps) = queue.pop_front().unwrap();
        if (y, x) == grid.e {
            return steps;
        }
        for (dy, dx) in &dirs {
            let (nx, ny) = (x as i32 + dx, y as i32+ dy);
            if nx >= 0 
            && nx < grid.n as i32 
            && ny >= 0 
            && ny < grid.m as i32 
            && !visited.contains(&(ny as usize, nx as usize)) {
                if grid.get(ny as usize, nx as usize) <= grid.get(y, x) + 1 {
                    visited.insert((ny as usize, nx as usize)); 
                    queue.push_back((ny as usize, nx as usize, steps + 1));
                }
            }
        }
    }
    0
}

/// Feels cheap, but to solve part 2 we're just going to invert this.
/// Start at E and return when we reach any 'a'. All it needs is flipping
/// the start point and the movement logic.
fn solve_part2(i: &str) -> i64 {
    let grid = Grid::from(i);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: VecDeque<(usize, usize, i64)> = VecDeque::new();
    queue.push_back((grid.e.0, grid.e.1, 0));
    visited.insert(grid.e);
    let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    while !queue.is_empty() {
        let (y, x, steps) = queue.pop_front().unwrap();
        if grid.get(y, x) == 'a' as usize {
            return steps;
        }
        for (dy, dx) in &dirs {
            let (nx, ny) = (x as i32 + dx, y as i32+ dy);
            if nx >= 0 
            && nx < grid.n as i32 
            && ny >= 0 
            && ny < grid.m as i32 
            && !visited.contains(&(ny as usize, nx as usize)) {
                if grid.get(ny as usize, nx as usize) >= grid.get(y, x) - 1 {
                    visited.insert((ny as usize, nx as usize)); 
                    queue.push_back((ny as usize, nx as usize, steps + 1));
                }
            }
        }
    }
    0
}
// BFS will find optimal path solution
// step can be in any dir that is down

struct Grid {
    m: usize,
    n: usize,
    grid: Vec<char>,
    s: (usize, usize), 
    e: (usize, usize)
}

impl Grid {
    fn from(i: &str) -> Self {
        let split_grid: Vec<&str> = i.split("\n").collect();
        let m = split_grid.len() - 1;
        let n = split_grid[0].len();
        let mut s = (0, 0);
        let mut e = (0, 0);
        let mut grid = vec![];
        for (i, line) in split_grid.iter().enumerate() {
            for (j, c) in line.chars().enumerate() {
                match c {
                    'S' => {
                        s = (i, j);
                        grid.push('a');
                    },
                    'E' => {
                        e = (i, j);
                        grid.push('z');
                    }, 
                    _ => grid.push(c),
                }
            }
        }

        Grid {m, n, grid, s, e}
    }

    fn get(&self, y: usize, x: usize) -> usize {
        self.grid[y * self.n + x] as usize
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = include_str!("test-input.txt");
    
    #[test]
    fn test_solve_part1() {
        assert_eq!(31, solve_part1(TEST_INPUT));
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(29, solve_part2(TEST_INPUT));
    }

    #[test]
    fn test_grid_get() {
        let grid = Grid::from(TEST_INPUT);
        assert_eq!('a' as usize, grid.get(0, 0));
        assert_eq!('a' as usize, grid.get(1, 0));
        assert_eq!('z' as usize, grid.get(2, 5));
    }
}
