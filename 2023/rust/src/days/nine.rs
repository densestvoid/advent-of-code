use std::{collections::HashSet, str::FromStr};

use log::info;

pub fn solve_part_1(input: &str) {
    let mut rope = Rope::new(1);
    let movements = parse_movements(input);

    for movement in movements {
        rope.update(movement);
    }

    info!("tail positions: {}", rope.tail_history.len());
}

pub fn solve_part_2(input: &str) {
    let mut rope = Rope::new(9);
    let movements = parse_movements(input);

    for movement in movements {
        rope.update(movement);
    }

    info!("tail positions: {}", rope.tail_history.len());
}

struct Rope {
    head: Position,
    knots: Vec<Position>,

    tail_history: HashSet<Position>,
}

impl Rope {
    fn new(length: usize) -> Self {
        let mut rope = Rope{
            head: Position{x: 0, y: 0},
            knots: vec![Position{x: 0, y: 0}; length],
            tail_history: HashSet::new(),
        };
        
        rope.tail_history.insert(Position{x: 0, y: 0});

        rope
    }

    fn update(&mut self, m: Movement) {
        match m {
            Movement::Up(d) => {
                for _ in 0..d {
                    self.step(Position { x: 0, y: 1 });
                }
            }
            Movement::Down(d) => {
                for _ in 0..d {
                    self.step(Position { x: 0, y: -1 });
                }
            }
            Movement::Left(d) => {
                for _ in 0..d {
                    self.step(Position { x: -1, y: 0 });
                }
            }
            Movement::Right(d) => {
                for _ in 0..d {
                    self.step(Position { x: 1, y: 0 });
                }
            }
        }
    }

    fn step (&mut self, head_update: Position) {
        self.head = Position{x: self.head.x+head_update.x, y: self.head.y+head_update.y};

        let mut prev_knot = Position{x: self.head.x, y: self.head.y};
        for knot in self.knots.iter_mut() {
            let x_change = prev_knot.x-knot.x;
            let y_change = prev_knot.y-knot.y;

            if x_change.abs() < 2 && y_change.abs() < 2 {
                break
            }

            knot.x += x_change.signum();
            knot.y += y_change.signum();
            prev_knot = Position{x: knot.x, y: knot.y};
        }

        match self.knots.last() {
            Some(knot) => {
                self.tail_history.insert(Position { x: knot.x, y: knot.y });
            }
            None => (),
        }
        
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Position {
    x: i32,
    y: i32,
}

fn parse_movements(input: &str) -> Vec<Movement> {
    let mut vec = Vec::new();

    for line in input.lines() {
        match line.parse() {
            Ok(m) => vec.push(m),
            Err(_) => continue,
        }
    }

    vec
}

enum Movement {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

struct ParseMovementError;

impl FromStr for Movement {
    type Err = ParseMovementError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(" ") {
            Some((direction, distance)) => {
                let distance = match distance.parse() {
                    Ok(d) => d,
                    Err(_) => return Err(ParseMovementError),
                };

                match direction {
                    "U" => Ok(Movement::Up(distance)),
                    "D" => Ok(Movement::Down(distance)),
                    "L" => Ok(Movement::Left(distance)),
                    "R" => Ok(Movement::Right(distance)),
                    _ => Err(ParseMovementError)
                }
            }
            None => Err(ParseMovementError),
        }
    }
}