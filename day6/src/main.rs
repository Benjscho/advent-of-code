use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    println!("The message is {} chars long", input.len());

    let result = get_marker(input, 4);
    println!("The start of packet marker is detected at {}", result);
    let result = get_marker(input, 14);
    println!("The start of message marker is detected at {}", result);
}

fn get_marker(message: &str, marker_len: usize) -> usize {
    let (potential_marker, remainder) = message.split_at(marker_len);
    let mut pm = potential_marker.to_string();
    let mut pos = marker_len;
    for c in remainder.chars() {
        if is_marker(&pm) {
            return pos;
        }
        pm.remove(0);
        pm.push(c);
        pos += 1;
    }
    pos
}

fn is_marker(pm: &str) -> bool {
    let mut a: HashSet<char> = HashSet::new();
    for c in pm.chars() {
        if a.contains(&c) {
            return false;
        } else {
            a.insert(c);
        }
    }
    true
}
