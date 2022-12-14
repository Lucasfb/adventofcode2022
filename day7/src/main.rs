use nom::{IResult, bytes::complete::*, combinator::map,branch::*,sequence::*};

#[derive(Debug,PartialEq)]
struct File {
    name: String,
    size: usize,
}

struct Folder {
    size: usize,
    children: Vec<FsEntry>
}

struct FsEntry {
    path: String,
    size: usize,
}

fn parse_path(i: &str) -> IResult<&str, String> {
    map(
        take_while1(|c: char| "abcdefghijklmnopqrstuvwxyz./".contains(c)),
        Into::into,
    )(i)
}

#[derive(Debug)]
struct Ls;

fn parse_ls(i: &str) -> IResult<&str,Ls> {
    map(tag("ls"),|_| Ls)(i)
}

#[derive(Debug)]
struct Cd(String);

fn parse_cd(i: &str) -> IResult<&str,Cd> {
    map(preceded(tag("cd "), parse_path),Cd)(i)
}



#[derive(Debug)]
enum Command {
    Cd(String),
    Ls,
}

fn parse_command(i: &str) -> IResult<&str,Command> {
    let (i,_) = tag("$")(i)?;
    alt((map(parse_ls,Into::into),map(parse_cd,Into::into)))(i)
}




impl From<Ls> for Command {
    fn from(_ls: Ls) -> Self {
        Command::Ls
    }
}
impl From<Cd> for Command {
    fn from(cd: Cd) -> Self {
        Command::Cd(cd.0)
    }
}
enum TerminalLine {
    Command,
    Entry,
}


fn main() {
    println!("Advent of Code 2022 - Day 3");
    let input = open_input_file();
    let total_size_to_delete = 0;
    
    // Parse input into a vec of the created structs


    // Go from the vec of structs to a tree data structure



    let current_node : FsEntry;

    println!("Answer for Part One: {}",total_size_to_delete);

}

fn open_input_file() -> String {
    let input_filename = "input.txt";
    fs_err::read_to_string(input_filename).unwrap()
}
