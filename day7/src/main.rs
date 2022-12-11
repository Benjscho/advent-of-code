use id_tree::*;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::{all_consuming, map, map_res},
    sequence::tuple,
    Finish, IResult,
};

fn main() {
    println!("Hello, world!");
    let input = include_str!("input.txt")
        .lines()
        .map(|l| all_consuming(parse_line)(l).finish().unwrap().1);

    let mut tree: Tree<FsItem> = TreeBuilder::new().build();

    let root: NodeId = tree
        .insert(
            Node::new(FsItem {
                name: "/".to_string(),
                size: 0,
            }),
            InsertBehavior::AsRoot,
        )
        .unwrap();
    let mut curr = root;

    for line in input {
        println!("{:?}", line);
        match line.unwrap() {
            Line::Item(item) => match item {
                Item::File(size, name) => {
                    let node = Node::new(FsItem { size, name });
                    tree.insert(node, InsertBehavior::UnderNode(&curr));
                }
                Item::Dir(_) => {}
            },
            Line::Command(command) => match command {
                Command::Move(target) => match target.as_str() {
                    "/" => {}
                    ".." => {
                        curr = tree.get(&curr).unwrap().parent().unwrap().clone();
                    }
                    _ => {
                        let node = Node::new(FsItem {
                            name: target.clone(),
                            size: 0,
                        });
                        curr = tree.insert(node, InsertBehavior::UnderNode(&curr)).unwrap();
                    }
                },
                Command::List => {}
            },
        }
    }

    let sum: u64 = tree
        .traverse_pre_order(tree.root_node_id().unwrap())
        .unwrap()
        .filter(|n| !n.children().is_empty())
        .map(|n| total_size(&tree, n))
        .filter(|&t| t <= 100_000)
        .sum();
    println!("Sum is {}", sum);

    let used_space =
        70_000_000 - total_size(&tree, tree.get(tree.root_node_id().unwrap()).unwrap());
    let min_space_to_free = 30_000_000 - used_space;

    let space_to_free: u64 = tree
        .traverse_pre_order(tree.root_node_id().unwrap())
        .unwrap()
        .filter(|n| !n.children().is_empty())
        .map(|n| total_size(&tree, n))
        .filter(|&t| t >= min_space_to_free)
        .min()
        .unwrap();
    println!("Space to free is {}", space_to_free);
}

fn total_size(tree: &Tree<FsItem>, node: &Node<FsItem>) -> u64 {
    let mut total = node.data().size;
    for child in node.children() {
        total += total_size(tree, tree.get(child).unwrap());
    }
    total
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

fn parse_file(i: &str) -> IResult<&str, Line> {
    let (input, (size, _, name)) = tuple((parse_number, tag(" "), parse_name))(i)?;
    let name = name.to_string();

    Ok((input, Line::Item(Item::File(size, name))))
}

fn parse_dir(i: &str) -> IResult<&str, Line> {
    let (i, _) = tag("dir ")(i)?;
    let (i, name) = take_while1(|c: char| c.is_alphanumeric())(i)?;

    Ok((i, Line::Item(Item::Dir(name.to_string()))))
}

fn parse_move_command(i: &str) -> IResult<&str, Line> {
    let (i, _) = tag("$ cd ")(i)?;
    let (i, target) = take_while1(|c: char| !c.is_whitespace())(i)?;
    Ok((i, Line::Command(Command::Move(target.to_string()))))
}

fn parse_list(i: &str) -> IResult<&str, Line> {
    let (i, _) = tag("$ ls")(i)?;
    Ok((i, Line::Command(Command::List)))
}

fn parse_line(i: &str) -> IResult<&str, Option<Line>> {
    alt((
        map(parse_move_command, Some),
        map(parse_list, Some),
        map(parse_file, Some),
        map(parse_dir, Some),
    ))(i)
}

// ---- Types
#[derive(Debug, PartialEq, Eq)]
enum Line {
    Item(Item),
    Command(Command),
}

#[derive(Debug, PartialEq, Eq)]
enum Item {
    File(u64, String),
    Dir(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Command {
    Move(String),
    List,
}

struct FsItem {
    name: String,
    size: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_parse() {
        let file_string = "14848514 b.txt";
        let expected = Line::Item(Item::File(14848514, "b.txt".to_string()));

        let (_, actual) = parse_file(file_string).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_dir_parse() {
        let dir_string = "dir a";
        let actual = Line::Item(Item::Dir("a".to_string()));
        let (_, expected) = parse_dir(dir_string).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_move_parse() {
        let move_strings = vec!["$ cd ..", "$ cd /", "$ cd e"];
        let expected_list = vec![
            Line::Command(Command::Move("..".to_string())),
            Line::Command(Command::Move("/".to_string())),
            Line::Command(Command::Move("e".to_string())),
        ];

        for (s, expected) in move_strings.iter().zip(expected_list.iter()) {
            let (_, actual) = parse_move_command(s).unwrap();
            assert_eq!(expected, &actual);
        }
    }

    #[test]
    fn test_list_parse() {
        let list = "$ ls";
        let expected = Line::Command(Command::List);
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
            "dir whatever",
        ];
        let expected_list = vec![
            Line::Command(Command::Move("..".to_string())),
            Line::Command(Command::Move("/".to_string())),
            Line::Command(Command::Move("e".to_string())),
            Line::Item(Item::File(1234, "myfile.txt".to_string())),
            Line::Command(Command::List),
            Line::Item(Item::Dir("whatever".to_string())),
        ];

        for (s, expected) in commands.iter().zip(expected_list.iter()) {
            let (_, actual) = parse_line(s).unwrap();
            assert_eq!(expected, &actual.unwrap());
        }
    }
}
