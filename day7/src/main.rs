use std::{collections::HashMap, fs::File};

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
//
fn parse_number(i: &str) -> IResult<&str, u64> {
    map_res(take_while1(|c: char| c.is_ascii_digit()), |s: &str| {
        s.parse::<u64>()
    })(i)
}

fn parse_name(i: &str) -> IResult<&str, &str> {
    let (i, name) = take_while1(|c: char| !c.is_whitespace())(i)?;
    Ok((i, name))
}

fn parse_file(i: &str) -> IResult<&str, LineInput> {
    let (input, (size, _, name)) = tuple((parse_number, tag(" "), parse_name))(i)?;
    let name = name.to_string();

    Ok((input, LineInput::File { size, name }))
}

fn parse_dir(i: &str) -> IResult<&str, LineInput> {
    let (i, _) = tag("dir ")(i)?;
    let (i, name) = take_while1(|c: char| c.is_alphanumeric())(i)?;

    Ok((i, LineInput::Dir {total_size: 0, objects: HashMap::new(), name: name.to_string()}))
}

fn parse_move_command(i: &str) -> IResult<&str, LineInput> {
    let (i, _) = tag("$ cd ")(i)?;
    let (i, target) = take_while1(|c: char| !c.is_whitespace())(i)?;
    Ok((i, LineInput::Move {target: target.to_string()}))
}

fn parse_list(i: &str) -> IResult<&str, LineInput> {
    let (i, _) = tag("$ ls")(i)?;
    Ok((i, LineInput::List))
}

fn parse_line(i: &str) -> IResult<&str, Option<LineInput>> {
    alt((
        map(parse_move_command, Some),
        map(parse_list, Some),
        map(parse_file, Some),
        map(parse_dir, Some),
    ))(i)
}

// ---- Types
#[derive(Clone, Debug, PartialEq, Eq)]
enum LineInput {
    File {
        size: u64,
        name: String
    },
    Dir {
        total_size: u64,
        objects: HashMap<String, LineInput>,
        name: String
    }, 
    Move {
        target: String
    },
    List
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
        let file = LineInput::File {size: 14848514, name: "b.txt".to_string()};

        let (_, parsed_file) = parse_file(file_string).unwrap();
        assert_eq!(file, parsed_file);
    }

    #[test]
    fn test_dir_parse() {
        let dir_string = "dir a";
        let actual = LineInput::Dir {total_size: 0, objects: HashMap::new(), name: "a".to_string()}; 
        let (_, expected) = parse_dir(dir_string).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_move_parse() {
        let move_strings = vec!["$ cd ..", "$ cd /", "$ cd e"];
        let expected_list = vec![
            LineInput::Move {target: "..".to_string()},
            LineInput::Move {target: "/".to_string()},
            LineInput::Move {target: "e".to_string()},
        ];

        for (s, expected) in move_strings.iter().zip(expected_list.iter()) {
            let (_, actual) = parse_move_command(s).unwrap();
            assert_eq!(expected, &actual); 
        }
    }

    #[test]
    fn test_list_parse() {
        let list = "$ ls";
        let expected = LineInput::List;
        let (_, actual) = parse_list(list).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_commands_parse() {
        let commands = vec![
            "$ cd ..", 
            "$ cd /", 
            "$ cd e",
            "1234 myfile.txt",
            "$ ls", 
            "dir whatever"
        ];
        let expected_list = vec![
            LineInput::Move {target: "..".to_string()},
            LineInput::Move {target: "/".to_string()},
            LineInput::Move {target: "e".to_string()},
            LineInput::File { size: 1234, name: "myfile.txt".to_string()},
            LineInput::List,
            LineInput::Dir { total_size: 0, objects: HashMap::new(), name: "whatever".to_string()}
        ];

        for (s, expected) in commands.iter().zip(expected_list.iter()) {

            let (_, actual) = parse_line(s).unwrap();
            assert_eq!(expected, &actual.unwrap()); 
        }
    }
}
