fn main() {
    let puzzle_input = include_str!("input.txt");
    let rounds = puzzle_input.lines().collect(); 
    let result = calc_score(&rounds); 
    println!("The answer is {}", result); 
    let result_two = calc_score_two(&rounds); 
    println!("The answer is {}", result_two); 
}   

fn calc_score(strategy: &Vec<&str>) -> u32 {
    return strategy.iter().fold(0, |x, s| x + game_score(s));
}

fn calc_score_two(strategy: &Vec<&str>) -> u32 {
    return strategy.iter().fold(0, |x, s| x + game_score_part_two(s));
}

fn game_score(game: &str) -> u32 {
    let mut score = 0;
    let mut split = game.split_whitespace(); 
    let a = split.next().unwrap();
    let b = split.next().unwrap();
    match b {
        "X" => score += 1,
        "Y" => score += 2,
        "Z" => score += 3,
        _ => ()
    }; 
    match (a, b) {
        ("A", "X") => score += 3, 
        ("A", "Y") => score += 6, 
        ("A", "Z") => score += 0, 
        ("B", "X") => score += 0, 
        ("B", "Y") => score += 3, 
        ("B", "Z") => score += 6, 
        ("C", "X") => score += 6, 
        ("C", "Y") => score += 0, 
        ("C", "Z") => score += 3, 
        _ => ()
    }
    score
}

fn game_score_part_two(game: &str) -> u32 {
    let mut score = 0;
    let mut split = game.split_whitespace(); 
    let a = split.next().unwrap();
    let b = split.next().unwrap();
    match (a, b) {
        ("A", "X") => score += 3, 
        ("A", "Y") => score += 4, 
        ("A", "Z") => score += 8, 
        ("B", "X") => score += 1, 
        ("B", "Y") => score += 5, 
        ("B", "Z") => score += 9, 
        ("C", "X") => score += 2, 
        ("C", "Y") => score += 6, 
        ("C", "Z") => score += 7, 
        _ => ()
    }
    score
}
