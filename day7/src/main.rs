use core::fmt;
use std::{collections::{HashMap, BTreeMap}, fs::File, cell::RefCell, rc::Rc};

use indexmap::IndexMap;
use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while1},
    combinator::{all_consuming, map, map_res, opt},
    sequence::{delimited, preceded, tuple},
    Finish, IResult,
};

fn main() {
    let input = include_str!("input.txt");

    let root = Rc::new(RefCell::new(Node::default()));
    let mut node = root.clone();

    let lines = input.lines().map(|l| all_consuming(parse_line)(l).finish().unwrap().1);

    for line in lines {
        match line.unwrap() {
            LineInput::List => {},
            LineInput::Move { target } => match target.as_str() {
                "/" => {},
                ".." => {
                    let parent = node.borrow().parent.clone().unwrap();
                    node = parent; 
                }
                _ => {
                    let child = node.borrow_mut().children.entry(target).or_default().clone();
                    node = child;
                }
            },
            LineInput::Dir { name } => {
                let entry = node.borrow_mut().children.entry(name).or_default().clone();
                entry.borrow_mut().parent = Some(node.clone()); 
            },
            LineInput::File { size, name } => {
                let entry = node.borrow_mut().children.entry(name).or_default().clone();
                entry.borrow_mut().size = size as usize;
                entry.borrow_mut().parent = Some(node.clone());
            }
        }
    }

    //let sum = all_dirs(root)
    //    .map(|d| d.borrow().total_size())
    //    .filter(|&s| s <= 100_000)
    //    .inspect(|s| {
    //        dbg!(s);
    //    })
    //    .sum::<u64>();
    //dbg!(sum);


    let total_used = root.borrow().total_size();
    let free = 70_000_000 - total_used; 
    dbg!(free);

    let delete_dir = all_dirs(root)
        .map(|d| d.borrow().total_size())
        .filter(|&s| s >= 30_000_000 - free)
        .min();
    dbg!(delete_dir); 
}

fn all_dirs(n: NodeHandle) -> Box<dyn Iterator<Item = NodeHandle>> {
    let children = n.borrow().children.values().cloned().collect::<Vec<_>>();

    Box::new(
        std::iter::once(n).chain(
            children
                .into_iter()
                .filter_map(|c| {
                    if c.borrow().is_dir() {
                        Some(all_dirs(c))
                    } else {
                        None
                    }
                })
                .flatten(),
        )
    )
}

impl Node {
    fn is_dir(&self) -> bool {
        self.size == 0 && !self.children.is_empty()
    }

    fn total_size(&self) -> u64 {
        self.children
            .values()
            .map(|child| child.borrow().total_size())
            .sum::<u64>()
            + self.size as u64
    }
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

    Ok((i, LineInput::Dir {name: name.to_string()}))
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
        name: String
    }, 
    Move {
        target: String
    },
    List
}

type NodeHandle = Rc<RefCell<Node>>;

#[derive(Default)]
struct Node {
    size: usize,
    children: IndexMap<String, NodeHandle>,
    parent: Option<NodeHandle>
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
            .field("size", &self.size)
            .field("children", &self.children)
            .finish()
    }
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
        let actual = LineInput::Dir {name: "a".to_string()}; 
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
            LineInput::Dir {  name: "whatever".to_string()}
        ];

        for (s, expected) in commands.iter().zip(expected_list.iter()) {

            let (_, actual) = parse_line(s).unwrap();
            assert_eq!(expected, &actual.unwrap()); 
        }
    }
}
