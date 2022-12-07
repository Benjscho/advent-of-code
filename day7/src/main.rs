use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while1},
    combinator::{all_consuming, map, map_res, opt},
    sequence::{delimited, preceded, tuple},
    Finish, IResult,
};

fn main() {
    println!("Hello, world!");
}



// -------- PARSING SECTION -------- //
fn parse_number(i: &str) -> IResult<&str, u64> {
    map_res(take_while1(|c: char| c.is_ascii_digit()), |s: &str| {
        s.parse::<u64>()
    })(i)
}

fn parse_name(i: &str) -> IResult<&str, &str> {
    let (i, name) = take_while1(|c: char| !c.is_whitespace())(i)?;
    Ok((i, name))
}

impl File {
    fn parse(i: &str) -> IResult<&str, File> {
        let (input, (size, _, name)) = tuple((parse_number, tag(" "), parse_name))(i)?;
        let name = name.to_string();

        Ok((input, File { size, name }))
    }
}

impl Dir {
    fn parse(i: &str) -> IResult<&str, Dir> {
        let (i, _) = tag("dir ")(i)?;
        let (i, name) = take_while1(|c: char| c.is_alphanumeric())(i)?;

        Ok((i, Dir {total_size: 0, objects: HashMap::new(), name: name.to_string()}))
    }
}

impl Move {
    fn parse(i: &str) -> IResult<&str, Move> {
        let (i, _) = tag("cd ")(i)?;
        let (i, target) = take_while1(|c: char| !c.is_whitespace())(i)?;
        Ok((i, Move {target: target.to_string()}))
    }
}


//fn parse_file_or_dir(i: &str) -> IResult<&str, Option<FileObject>> {
//    alt((map(parse_file, Some), map(parse_dir, Some)))(i)
//}

// ---- Types
#[derive(Clone, Debug, PartialEq, Eq)]
enum FileObject {
    File,
    Dir
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct File {
    size: u64,
    name: String
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Dir {
    total_size: u64,
    objects: HashMap<String, FileObject>,
    name: String
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Command {
    Move,
    List
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Move {
    target: String
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_file_parse() {
        let file_string = "14848514 b.txt";
        let file = File {size: 14848514, name: "b.txt".to_string()};

        let (_, parsed_file) = File::parse(file_string).unwrap();
        assert_eq!(file, parsed_file);
    }

    #[test]
    fn test_dir_parse() {
        let dir_string = "dir a";
        let actual = Dir {total_size: 0, objects: HashMap::new(), name: "a".to_string()}; 
        let (_, expected) = Dir::parse(dir_string).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_move_parse() {
        let move_strings = vec!["cd ..", "cd /", "cd e"];
        let actual = vec![
            Move {target: "..".to_string()},
            Move {target: "/".to_string()},
            Move {target: "e".to_string()},
        ];

        for (s, m) in move_strings.iter().zip(actual.iter()) {
            let (_, tm) = Move::parse(s).unwrap();
            assert_eq!(m, &tm); 
        }
    }
}
