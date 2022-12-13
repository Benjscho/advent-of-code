fn main() {
    println!("Hello, world!");
}

fn solve_part1(i: &str) -> i64 {
    unimplemented!()
}

#[cfg(test)]
mod test {
    use super::*;
    static TEST_INPUT: &str = include_str!("test-input.txt"); 

    #[test]
    fn test_part1() {
        assert_eq!(0, solve_part1(TEST_INPUT)); 
    }
}
