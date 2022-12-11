fn main() {
    let input = include_str!("input.txt");
    let result = input.lines().fold(0, |x, pair| x + full_overlap(pair));
    println!("Number of completely overlapping pairs: {}", result);
    let result = input.lines().fold(0, |x, pair| x + any_overlap(pair));
    println!("Number of partially overlapping pairs: {}", result);
}

fn full_overlap(s: &str) -> u32 {
    let (a, b) = s.split_once(',').unwrap();
    let (sa, ea) = a.split_once('-').unwrap();
    let (sb, eb) = b.split_once('-').unwrap();
    let sa = sa.parse::<u32>().unwrap();
    let sb = sb.parse::<u32>().unwrap();
    let ea = ea.parse::<u32>().unwrap();
    let eb = eb.parse::<u32>().unwrap();

    u32::from((sa <= sb && ea >= eb) || (sb <= sa && eb >= ea))
}

fn any_overlap(s: &str) -> u32 {
    let (a, b) = s.split_once(',').unwrap();
    let (sa, ea) = a.split_once('-').unwrap();
    let (sb, eb) = b.split_once('-').unwrap();
    let sa = sa.parse::<u32>().unwrap();
    let sb = sb.parse::<u32>().unwrap();
    let ea = ea.parse::<u32>().unwrap();
    let eb = eb.parse::<u32>().unwrap();

    // Pair doesn't overlap when the end of the first section is before
    // the start of the other.
    if ea < sb || sa > eb {
        0
    } else {
        1
    }
}
