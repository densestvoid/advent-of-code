use std::{str::FromStr, cmp::Ordering};

use log::info;

#[derive(Debug)]
struct Assignment {
    start: u32,
    end: u32,
}

struct ParseAssignmentError;

impl FromStr for Assignment {
    type Err = ParseAssignmentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = match s.split_once("-") {
            Some(value) => value,
            None => return Err(ParseAssignmentError),
        };

        let start: u32 = match first.parse() {
            Ok(d) => d,
            Err(_) => return Err(ParseAssignmentError),
        };

        let end: u32 = match second.parse() {
            Ok(d) => d,
            Err(_) => return Err(ParseAssignmentError),
        };

        Ok(Assignment { start: start, end: end })
    }
}

struct AssignmentPair (Assignment, Assignment);

impl AssignmentPair {
    fn full_overlap(&self) -> bool {
        let start_cmp = self.0.start.cmp(&self.1.start);
        let end_cmp = self.0.end.cmp(&self.1.end);

        match (start_cmp, end_cmp) {
            (Ordering::Less, Ordering::Less) => false,
            (Ordering::Greater, Ordering::Greater) => false,
            (_, _) => true,
        }
    }

    fn partial_overlap(&self) -> bool {
        let start_cmp = self.0.start.cmp(&self.1.end);
        let end_cmp = self.0.end.cmp(&self.1.start);

        match (start_cmp, end_cmp) {
            (Ordering::Greater, _) => false,
            (_, Ordering::Less) => false,
            (_, _) => true,
        }
    }
}

fn detail_assignments(input: &str) -> Vec<AssignmentPair> {
    let mut vec = Vec::new();

    for line in input.lines() {
        let (first, second) = match line.split_once(",") {
            Some(value) => value,
            None => continue,
        };

        let first = match first.parse::<Assignment>() {
            Ok(assignment) => assignment,
            Err(_) => continue,
        };

        let second = match second.parse::<Assignment>() {
            Ok(assignment) => assignment,
            Err(_) => continue,
        };

        vec.push(AssignmentPair(first, second));
    }

    vec
}

fn count_fully_contained(pairs: Vec<AssignmentPair>) -> u32 {
    let mut count = 0;

    for pair in pairs {
        if pair.full_overlap() {
            count += 1;
        }
    }

    count
}

fn count_partially_contained(pairs: Vec<AssignmentPair>) -> u32 {
    let mut count = 0;

    for pair in pairs {
        if pair.partial_overlap() {
            count += 1;
        }
    }

    count
}

pub fn solve_part_1(input: &str) {
    let pairs = detail_assignments(input);
    let count = count_fully_contained(pairs);
    info!("overlapping pairs: {}", count);
}

pub fn solve_part_2(input: &str) {
    let pairs = detail_assignments(input);
    let count = count_partially_contained(pairs);
    info!("overlapping pairs: {}", count);
}