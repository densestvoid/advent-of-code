use std::{str::FromStr, collections::HashMap};

use log::{info, debug, error};

pub fn solve_part_1(input: &str) {
    let mut dir = Dir::new(String::from("/"), None);

    for line in input.lines() {
        let (prefix, line) = match line.split_once(" ") {
            Some(split) => split,
            None => panic!("line without prefix"),
        };

        match prefix {
            "$" => {
                match line.parse::<Command>() {
                    Ok(command) => match command {
                        Command::CD(cd_arg) => match cd_arg {
                            CDArg::Top => {
                                debug!("cd /");
                                dir = dir.top()
                            }
                            CDArg::Out => {
                                debug!("cd ..");
                                dir = dir.parent()
                            }
                            CDArg::In(s) => {
                                debug!("cd {}", s);
                                dir = dir.child(s);
                            }
                        }
                        Command::LS => debug!("ls"),
                    }
                    Err(e) => panic!("unable to parse command: `{}`", e.input),
                }
            }
            "dir" => (),
            _ => {
                match prefix.parse() {
                    Ok(size) => dir.file(String::from(line), size),
                    Err(e) => error!("{}", e),
                }
            },
        }
    }

    dir = dir.top();

    let dir_sizes = dir.dir_sizes();
    info!("total size: {}", dir_sizes.iter().filter(|s| **s <= 100_000).sum::<u32>())
}

pub fn solve_part_2(input: &str) {

    let mut dir = Dir::new(String::from("/"), None);

    for line in input.lines() {
        let (prefix, line) = match line.split_once(" ") {
            Some(split) => split,
            None => panic!("line without prefix"),
        };

        match prefix {
            "$" => {
                match line.parse::<Command>() {
                    Ok(command) => match command {
                        Command::CD(cd_arg) => match cd_arg {
                            CDArg::Top => {
                                debug!("cd /");
                                dir = dir.top()
                            }
                            CDArg::Out => {
                                debug!("cd ..");
                                dir = dir.parent()
                            }
                            CDArg::In(s) => {
                                debug!("cd {}", s);
                                dir = dir.child(s);
                            }
                        }
                        Command::LS => debug!("ls"),
                    }
                    Err(e) => panic!("unable to parse command: `{}`", e.input),
                }
            }
            "dir" => (),
            _ => {
                match prefix.parse() {
                    Ok(size) => dir.file(String::from(line), size),
                    Err(e) => error!("{}", e),
                }
            },
        }
    }

    dir = dir.top();
    let required_space = 30_000_000 - (70_000_000 - dir.size());
    info!("required space: {}", required_space);


    let dir_sizes = dir.dir_sizes();
    info!("total size: {}", match dir_sizes.iter().filter(|s| **s >= required_space).min() {
        Some(size) => size,
        None => &0,
    })
}

#[derive(Debug)]
struct Dir {
    name: String,
    parent: Option<Box<Self>>,
    children: HashMap<String, Box<Self>>,
    files: HashMap<String, u32>,
}

impl Dir {
    fn new(name: String, parent: Option<Box<Dir>>) -> Self {
        Self {
            name: name,
            parent: parent,
            children: HashMap::new(),
            files: HashMap::new(),
        }
    }

    fn file(&mut self, name: String, size: u32) {
        self.files.entry(name).or_insert(size);
    }

    fn child(mut self, name: String) -> Self {
        match self.children.remove(&name) {
            Some(mut child) => {
                child.parent = Some(Box::new(self));
                *child
            }
            None => {
                Dir::new(name, Some(Box::new(self)))
            }
        }
    }

    fn parent(mut self) -> Self {
        match self.parent {
            Some(mut parent) => {
                self.parent = None;
                parent.children.insert(self.name.clone(), Box::new(self));
                *parent
            }
            None => self,
        }
    }

    fn top(self) -> Self {
        let mut curr = self;
        loop {
            match curr.parent {
                Some(mut parent) => {
                    curr.parent = None;
                    parent.children.insert(curr.name.clone(), Box::new(curr));
                    curr = *parent
                }
                None => return curr,
            }
        }
    }
    
    fn size(&self) -> u32 {
        let mut sum = 0;

        for (_, size) in &self.files {
            sum += size;
        }

        for (_, dir) in &self.children {
            sum += dir.size();
        }

        sum
    }

    fn dir_sizes(&self) -> Vec<u32> {
        let mut vec = Vec::new();

        vec.push(self.size());

        for (_, child) in &self.children {
            vec.append(&mut child.dir_sizes());
        }

        vec
    }
}
 

enum CDArg {
    In(String),
    Out,
    Top,
}

enum Command {
    CD(CDArg),
    LS
}

struct ParseCommandError{
    input: String,
}

impl FromStr for Command {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(" ") {
            Some((command, args)) => match (command, args) {
                ("cd", "..") => Ok(Command::CD(CDArg::Out)),
                ("cd", "/") => Ok(Command::CD(CDArg::Top)),
                ("cd", _) => Ok(Command::CD(CDArg::In(args.to_string()))),
                (_, _) => Err(ParseCommandError{input: String::from(s)}),
            }
            None => match s {
                "ls" => Ok(Command::LS),
                _ => Err(ParseCommandError{input: String::from(s)}),
            }
        }
    }
}