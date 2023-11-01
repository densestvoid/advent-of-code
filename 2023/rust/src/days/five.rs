use core::panic;
use std::num::ParseIntError;

pub fn solve_part_1(input : &str) {
    let (mut stacks, moves) = parse_input(input);

    for m in moves {
        for _ in 0..m.count {
            match stacks[m.from_stack-1].pop() {
                Some(cargo) => stacks[m.to_stack-1].push(cargo),
                None => panic!("no cargo on stack"),
            }
        }
    }

    let mut code = String::new();
    for mut stack in stacks {
        match stack.pop() {
            Some(c) => code.push(c),
            None => (),
        };
    }

    println!("code: {}", code)
}

pub fn solve_part_2(input : &str) {
    let (mut stacks, moves) = parse_input(input);

    for m in moves {
        let from = match stacks.get_mut(m.from_stack-1) {
            Some(stack) => stack,
            None => panic!("index should exist"),
        };

        let mut drained = Vec::from_iter(from.drain(from.len()-m.count..));

        let to = match stacks.get_mut(m.to_stack-1) {
            Some(stack) => stack,
            None => panic!("index should exist"),
        };

        to.append(&mut drained);
    }

    let mut code = String::new();
    for mut stack in stacks {
        match stack.pop() {
            Some(c) => code.push(c),
            None => (),
        };
    }

    println!("code: {}", code)
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Move>) {
    let mut lines = input.lines();

    let mut drawing_lines = Vec::new();
    loop {
        let line = match lines.next() {
            Some(s) => s,
            None => break,
        };

        if line.is_empty() {
            break
        }

        drawing_lines.push(line);
    }

    let mut move_lines = Vec::new();
    loop {
        let line = match lines.next() {
            Some(s) => s,
            None => break,
        };

        if line.is_empty() {
            break
        }

        move_lines.push(line);
    }

    (parse_drawing(drawing_lines), parse_moves(move_lines))
}

fn parse_drawing(mut lines: Vec<&str>) -> Vec<Vec<char>>{
    let mut stacks = new_stacks(match lines.pop() {
        Some(s) => s,
        None => panic!("no stack base"),
    });

    while lines.len() > 0 {
        match lines.pop() {
            Some(s) => add_crates(&mut stacks, s),
            None => panic!("shouldn't get here"),
        };
    }

    stacks
}

fn new_stacks(input: &str) -> Vec<Vec<char>>{
    let mut vec = Vec::new();

    for _ in input.split_whitespace() {
        vec.push(Vec::new());
    }

    vec
}

fn add_crates(stacks: &mut Vec<Vec<char>>, input: &str) {
    for (i, stack) in stacks.iter_mut().enumerate() {
        add_crate(stack, &input[i*4..i*4+3]);
    }
}

fn add_crate(stack: &mut Vec<char>, input: &str) {
    match input.chars().nth(1) {
        Some(c) => match c {
            ' ' => (),
            c => stack.push(c),
        },
        None => panic!("shouldn't get here"),
    }
}

struct Move {
    count: usize,
    from_stack: usize,
    to_stack: usize,
}

fn parse_moves(lines: Vec<&str>) -> Vec<Move> {
    let mut vec = Vec::new();

    for line in lines {
        match parse_move(line) {
            Ok(m) => vec.push(m),
            Err(_) => continue,
        }
    }

    vec
}

fn parse_move(line: &str) -> Result<Move, ParseIntError> {
    let words: Vec<&str> = line.split_whitespace().collect();
    Ok(Move { count: words[1].parse()?, from_stack: words[3].parse()?, to_stack: words[5].parse()? })
}