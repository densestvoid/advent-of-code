use std::{str::FromStr, collections::VecDeque};

use log::{info, debug};

pub fn solve_part_1(input: &str) {
    let mut monkeys = parse_monkeys(input);

    for _ in 0..20 {
        for monkey in 0..monkeys.len() {
            while let Some((to, item)) = monkeys[monkey].process_divisor() {
                monkeys[to as usize].catch(item);
            }
        }
    }

    monkeys.sort();
    let monkey_business: u64 = monkeys.iter().map(|m| m.inspections).rev().take(2).product();
    info!("monkey_business: {}", monkey_business);
}

pub fn solve_part_2(input: &str) {
    let mut monkeys = parse_monkeys(input);

    let divisor: u64 = monkeys.iter().map(|m| m.factor).product();

    for round in 0..10000 {
        debug!("round: {}", round);
        for monkey in 0..monkeys.len() {
            while let Some((to, item)) = monkeys[monkey].process_modulo(divisor) {
                monkeys[to as usize].catch(item);
            }
        }
    }

    monkeys.sort();
    for monkey in &monkeys {
        debug!("{}, inspections: {}", monkey.number, monkey.inspections);
    }
    let monkey_business: u64 = monkeys.iter().map(|m| m.inspections).rev().take(2).product();
    info!("monkey_business: {}", monkey_business);
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    let mut vec = Vec::new();

    let monkey_notes = input.split("\n\n");
    for notes in monkey_notes {
        match notes.parse::<Monkey>() {
            Ok(m) => vec.push(m),
            Err(e) => debug!("{}", e.reason),
        }
    }

    vec
}

struct Monkey {
    number: u64,
    items: VecDeque<u64>,
    operation: Operation,
    factor: u64,
    factor_true: u64,
    factor_false: u64,

    inspections: u64,
}

impl Monkey {
    fn process_divisor(&mut self) -> Option<(u64, u64)> {
        match self.items.pop_front() {
            Some(mut item) => {
                self.inspections += 1;

                item = match self.operation {
                    Operation::AdditionSelf => item.saturating_add(item),
                    Operation::Addition(val) => item.saturating_add(val),
                    Operation::MultiplicationSelf => (item).saturating_mul(item),
                    Operation::Multiplication(val) => (item).saturating_mul(val),
                };

                item /= 3;

                let m = if item % self.factor == 0 {
                    self.factor_true
                } else {
                    self.factor_false
                };

                Some((m, item))
            }
            None => None,
        }
    }

    fn process_modulo(&mut self, modulo: u64) -> Option<(u64, u64)> {
        match self.items.pop_front() {
            Some(mut item) => {
                self.inspections += 1;

                item = match self.operation {
                    Operation::AdditionSelf => item.saturating_add(item),
                    Operation::Addition(val) => item.saturating_add(val),
                    Operation::MultiplicationSelf => (item%modulo).saturating_mul(item%modulo),
                    Operation::Multiplication(val) => (item%modulo).saturating_mul(val%modulo),
                };

                let m = if item % self.factor == 0 {
                    self.factor_true
                } else {
                    self.factor_false
                };

                Some((m, item))
            }
            None => None,
        }
    }

    fn catch(&mut self, item: u64) {
        self.items.push_back(item)
    }
}

impl PartialEq for Monkey {
    fn eq(&self, other: &Self) -> bool {
        self.inspections == other.inspections
    }
}

impl PartialOrd for Monkey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.inspections.cmp(&other.inspections))
    }
}

impl Eq for Monkey {}

impl Ord for Monkey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inspections.cmp(&other.inspections)
    }
}

struct ParseMonkeyError {
    reason: String,
}

impl FromStr for Monkey {
    type Err = ParseMonkeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();

        if lines.len() != 6 {
            return Err(ParseMonkeyError{reason: String::from("line count doesn't match")});
        }

        let number = match lines.get(0) {
            Some(line) => match line.strip_prefix("Monkey ") {
                Some(line) => match line.strip_suffix(":") {
                    Some(line) => match line.parse::<u64>() {
                        Ok(d) => d,
                        Err(_) => return Err(ParseMonkeyError{reason: String::from("failed to parse number")}),
                    }
                    None => return Err(ParseMonkeyError{reason: String::from("failed to strip number suffix")}),
                }
                None => return Err(ParseMonkeyError{reason: String::from("failed to strip number prefix")}),
            }
            None => return Err(ParseMonkeyError{reason: String::from("failed to get number line")}),
        };

        let items = match lines.get(1) {
            Some(line) => match line.strip_prefix("  Starting items: ") {
                Some(line) => {
                    let mut vec = VecDeque::new();
    
                    for num in line.split(", ") {
                        vec.push_back(match num.parse::<u64>() {
                            Ok(d) => d,
                            Err(_) => continue,
                        });
                    }
    
                    vec
                }
                None => return Err(ParseMonkeyError{reason: String::from("failed to strip items prefix")}),
            }
            None => return Err(ParseMonkeyError{reason: String::from("failed to get items line")}),
        };

        let operation = match lines.get(2) {
            Some(line) => match line.parse::<Operation>() {
                Ok(op) => op,
                Err(_) => return Err(ParseMonkeyError{reason: String::from("failed to parse operation")}),
            }
            None => return Err(ParseMonkeyError{reason: String::from("failed to get operation line")}),
        };

        let factor = match lines.get(3) {
            Some(line) => match line.strip_prefix("  Test: divisible by ") {
                Some(line) => match line.parse::<u64>() {
                    Ok(d) => d,
                    Err(_) => return Err(ParseMonkeyError{reason: String::from("failed to parse factor")}),
                }
                None => return Err(ParseMonkeyError{reason: String::from("failed to strip factor prefix")}),
            }
            None => return Err(ParseMonkeyError{reason: String::from("failed to get factor line")}),
        };

        let factor_true = match lines.get(4) {
            Some(line) => match line.strip_prefix("    If true: throw to monkey ") {
                Some(line) => match line.parse::<u64>() {
                    Ok(d) => d,
                    Err(_) => return Err(ParseMonkeyError{reason: String::from("failed to parse factor true")}),
                }
                None => return Err(ParseMonkeyError{reason: String::from("failed to strip factor true")}),
            }
            None => return Err(ParseMonkeyError{reason: String::from("failed to get factor true line")}),
        };

        let factor_false = match lines.get(5) {
            Some(line) => match line.strip_prefix("    If false: throw to monkey ") {
                Some(line) => match line.parse::<u64>() {
                    Ok(d) => d,
                    Err(_) => return Err(ParseMonkeyError{reason: String::from("failed to parse factor false")}),
                }
                None => return Err(ParseMonkeyError{reason: String::from("failed to strip factor false")}),
            }
            None => return Err(ParseMonkeyError{reason: String::from("failed to get factor false line")}),
        };

        Ok(Monkey{
            number: number,
            items: items,
            operation: operation,
            factor: factor,
            factor_true: factor_true,
            factor_false: factor_false,

            inspections: 0,
        })
    }
}

enum Operation {
    AdditionSelf,
    Addition(u64),
    MultiplicationSelf,
    Multiplication(u64),
}

struct ParseOperationError;

impl FromStr for Operation {
    type Err = ParseOperationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.strip_prefix("  Operation: new = old ") {
            Some(s) => match s.split_once(" ") {
                Some((operator, value)) => match value.parse::<u64>() {
                    Ok(d) => match operator {
                        "+" => Ok(Operation::Addition(d)),
                        "*" => Ok(Operation::Multiplication(d)),
                        _ => return Err(ParseOperationError),
                    }
                    Err(_) => {
                        if value == "old" {
                            match operator {
                                "+" => return Ok(Operation::AdditionSelf),
                                "*" => return Ok(Operation::MultiplicationSelf),
                                _ => return Err(ParseOperationError),
                            };
                        }
                        Err(ParseOperationError)
                    }
                }
                None => return Err(ParseOperationError),
            }
            None => return Err(ParseOperationError),
        }
    }
}