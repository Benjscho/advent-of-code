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

fn parse_number(i: &str) -> IResult<&str, u64> {
    map_res(take_while1(|c: char| c.is_ascii_digit()), |s: &str| {
        s.parse::<u64>()
    })(i)
}

fn parse_name(i: &str) -> IResult<&str, &str> {
    let (i, name) = take_while1(|c: char| c.is_alphanumeric())(i)?;
    Ok((i, name))
}

fn parse_file(i: &str) -> IResult<&str, File> {
    let (input, (size, _, name)) = tuple((parse_number, tag(" "), parse_name))(i)?;
    let name = name.to_string();

    Ok((input, File { size, name }))
}

fn parse_dir(i: &str) -> IResult<&str, Dir> {
    let (i, _) = tag("dir ")(i)?;
    let (i, name) = take_while1(|c: char| c.is_alphanumeric())(i)?;

    Ok((i, Dir {total_size: 0, objects: HashMap::new(), name: name.to_string()}))
}

//fn parse_command(i: &str) -> IResult<&str, 

//fn parse_file_or_dir(i: &str) -> IResult<&str, Option<FileObject>> {
//    alt((map(parse_file, Some), map(parse_dir, Some)))(i)
//}

// ---- Types
#[derive(Clone)]
enum FileObject {
    File,
    Dir
}

#[derive(Clone, Debug)]
struct File {
    size: u64,
    name: String
}

#[derive(Clone)]
struct Dir {
    total_size: u64,
    objects: HashMap<String, FileObject>,
    name: String
}

