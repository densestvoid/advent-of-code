use std::collections::HashSet;

use log::info;

const PRIORITY_STRING: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn priority(c: &char) -> u32 {
    match PRIORITY_STRING.find(*c) {
        Some(index) => u32::try_from(index + 1).unwrap(),
        None => 0,
    }
}

fn letters(s: &str) -> HashSet<char> {
    let mut set = HashSet::new();

    for c in s.chars() {
        set.insert(c);
    }

    set
}

fn itemize(s: &str) -> Vec<char> {
    let mut v = Vec::new();

    for line in s.lines() {
        let (part_a, part_b) = line.split_at(line.len()/2);
        let letters_a = letters(part_a);
        let letters_b = letters(part_b);
        v.push(letters_a.intersection(&letters_b).next().unwrap().clone())
    }

    v
}

fn badges(s: &str) -> Vec<char> {
    let mut v = Vec::new();

    let mut lines = s.lines();
    loop {
        let letters_a = match lines.next() {
            Some(line) => letters(line),
            None => break,
        };
        let letters_b = letters(lines.next().unwrap());
        let letters_c = letters(lines.next().unwrap());

        for c in letters_a {
            if letters_b.contains(&c) && letters_c.contains(&c) {
                v.push(c);
                break
            }
        }
    }

    v
}

pub fn solve_part_1(input: &str) {
    let item_errors = itemize(&input);

    let mut sum = 0;
    for err in item_errors {
        sum += priority(&err);
    }

    info!("priority: {}", sum)
}

pub fn solve_part_2(input: &str) {
    let bs = badges(&input);

    let mut sum = 0;
    for err in bs {
        sum += priority(&err);
    }

    info!("priority: {}", sum)
}