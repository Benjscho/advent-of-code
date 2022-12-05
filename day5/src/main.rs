fn main() {
    let puzzle_input = include_str!("input.txt");
    let (init_state, movements) = puzzle_input.split_once("\n\n").unwrap();

    let mut ship: Ship = Ship::new(init_state);
    ship.move_crates(movements);
    let result = ship.get_top_of_stacks();
    println!("The answer is {}", result);
    let mut ship: Ship = Ship::new(init_state);
    ship.move_crates_two(movements);
    let result = ship.get_top_of_stacks();
    println!("The revised answer is {}", result);
}

struct Ship {
    crates: Vec<Vec<char>>,
}

impl Ship {
    fn new(ship_state: &str) -> Self {
        let mut state_lines: Vec<&str> = ship_state.lines().collect();
        let num_lines = state_lines
            .pop()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        println!("Found {} crates", num_lines);
        let mut crates: Vec<Vec<char>> = vec![Vec::new(); num_lines];

        state_lines.reverse();
        for mut state in state_lines {
            for i in 0..num_lines {
                let c;
                (c, state) = state.split_at(3);
                if state.len() > 0 {
                    (_, state) = state.split_at(1); // Split at keeps the midpoint so we need to
                }
                // discard that
                let letter = c.chars().nth(1).unwrap();
                if !letter.is_whitespace() {
                    println!("Placing {} in {}", &letter, &i);
                    crates.get_mut(i).unwrap().push(letter);
                }
            }
        }

        Self { crates }
    }

    fn get_top_of_stacks(&self) -> String {
        self.crates
            .iter()
            .fold("".to_string(), |x, c| x + &c.last().unwrap().to_string())
    }

    fn move_crates(&mut self, procedure: &str) {
        let moves = procedure.lines();
        for m in moves {
            let mut m = m.split_whitespace();
            let n = m.nth(1).unwrap().parse::<usize>().unwrap();
            let source = m.nth(1).unwrap().parse::<usize>().unwrap() - 1;
            let dest = m.nth(1).unwrap().parse::<usize>().unwrap() - 1;
            println!("Moving {} from {} to {}", &n, &source, &dest);
            for _ in 0..n {
                let c = self.crates.get_mut(source).unwrap().pop().unwrap();
                self.crates.get_mut(dest).unwrap().push(c);
            }
        }
    }

    /// This is very hacky
    /// Just takes the move crates function and instead of pushing them directly
    /// it puts them in temp storage, reverses that, then pushes each of
    /// those to the new column. As it's rust it's still fast.
    fn move_crates_two(&mut self, procedure: &str) {
        let moves = procedure.lines();
        for m in moves {
            let mut m = m.split_whitespace();
            let n = m.nth(1).unwrap().parse::<usize>().unwrap();
            let source = m.nth(1).unwrap().parse::<usize>().unwrap() - 1;
            let dest = m.nth(1).unwrap().parse::<usize>().unwrap() - 1;
            println!("Moving {} from {} to {}", &n, &source, &dest);
            let mut temp_storage: Vec<char> = Vec::new();
            for _ in 0..n {
                temp_storage.push(self.crates.get_mut(source).unwrap().pop().unwrap());
            }
            temp_storage.reverse();
            for c in temp_storage {
                self.crates.get_mut(dest).unwrap().push(c);
            }
        }

    }
}
