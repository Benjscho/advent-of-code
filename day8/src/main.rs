use std::cmp::max;

fn main() {
    println!("Hello, world!");
    let input = include_str!("input.txt");
    let solution = solve(input);
    println!("{} trees are visible", solution); 
    let best_view = scenic_score(input);
    println!("{} is the highest score", best_view); 
}

fn solve(input: &str) -> usize {
    let trees: Vec<&str> = input.lines().collect();
    let trees: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let m = trees.len();
    let n = trees.get(0).unwrap().len();
    let mut visible = (2 * m) + (2 * n) - 4;

    for i in 1..m-1 {
        for j in 1..n-1 {
            let tree = trees.get(i).unwrap().get(j).unwrap();
            let mut pos = i; 
            let mut vis = true; 
            while pos > 0 && vis {
                pos -= 1;
                if trees.get(pos).unwrap().get(j).unwrap() >= tree {
                    vis = false; 
                }
            }
            if vis {
                visible += 1;
                continue;
            }

            let mut pos = i; 
            let mut vis = true; 
            while pos < m - 1 && vis {
                pos += 1;
                if trees.get(pos).unwrap().get(j).unwrap() >= tree {
                    vis = false; 
                }
            }
            if vis {
                visible += 1;
                continue;
            }

            let mut pos = j; 
            let mut vis = true; 
            while pos > 0 && vis {
                pos -= 1;
                if trees.get(i).unwrap().get(pos).unwrap() >= tree {
                    vis = false; 
                }
            }
            if vis {
                visible += 1;
                continue;
            }

            let mut pos = j; 
            let mut vis = true; 
            while pos < n - 1 && vis {
                pos += 1;
                if trees.get(i).unwrap().get(pos).unwrap() >= tree {
                    vis = false; 
                }
            }
            if vis {
                visible += 1;
                continue;
            }
        }
    }
    visible
}

fn scenic_score(input: &str) -> usize {
    let trees: Vec<&str> = input.lines().collect();
    let trees: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let m = trees.len();
    let n = trees.get(0).unwrap().len();
    let mut max_scenic_score = 0;

    for i in 0..m {
        for j in 0..n {
            let tree = trees.get(i).unwrap().get(j).unwrap();
            let mut pos = i; 
            let mut vis = true; 
            let mut up = 0;
            while pos > 0 && vis {
                pos -= 1;
                if trees.get(pos).unwrap().get(j).unwrap() >= tree {
                    vis = false; 
                }
                up += 1;
            }

            let mut pos = i; 
            let mut vis = true; 
            let mut down = 0;
            while pos < m - 1 && vis {
                pos += 1;
                if trees.get(pos).unwrap().get(j).unwrap() >= tree {
                    vis = false; 
                }
                down += 1;
            }

            let mut pos = j; 
            let mut vis = true; 
            let mut left = 0;
            while pos > 0 && vis {
                pos -= 1;
                if trees.get(i).unwrap().get(pos).unwrap() >= tree {
                    vis = false; 
                }
                left += 1;
            }

            let mut pos = j; 
            let mut vis = true; 
            let mut right = 0;
            while pos < n - 1 && vis {
                pos += 1;
                if trees.get(i).unwrap().get(pos).unwrap() >= tree {
                    vis = false; 
                }
                right += 1;
            }
            let score = left * right * up * down;
            max_scenic_score = max(max_scenic_score, score); 
        }
    }
    max_scenic_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let test_input = include_str!("test-input.txt");
        assert_eq!(solve(test_input), 21);
    }

    #[test]
    fn test_scenic_score() {
        let test_input = include_str!("test-input.txt");
        assert_eq!(scenic_score(test_input), 8);
    }
}
