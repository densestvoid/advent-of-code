use std::str::FromStr;

use log::info;

trait FromABC {
    fn from_abc(c: char) -> Self;
}

trait FromXYZ {
    fn from_xyz(c: char) -> Self;
}

#[derive(Clone,Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn score(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn from_opponent_and_outcome(opponent: &Shape, out: &Outcome) -> Shape {
        match (opponent, out) {
            (_, Outcome::Draw) => *opponent,
            (Shape::Rock, Outcome::Lose) => Shape::Scissors,
            (Shape::Rock, Outcome::Win) => Shape::Paper,
            (Shape::Paper, Outcome::Lose) => Shape::Rock,
            (Shape::Paper, Outcome::Win) => Shape::Scissors,
            (Shape::Scissors, Outcome::Lose) => Shape::Paper,
            (Shape::Scissors, Outcome::Win) => Shape::Rock,
        }
    }
}

impl FromABC for Shape {
    fn from_abc(c: char) -> Shape {
        match c {
            'A' => Shape::Rock,
            'B' => Shape::Paper,
            'C' => Shape::Scissors,
            _ => panic!("invalid opponenet code: {}", c)
        }
    }
}

impl FromXYZ for Shape {
    fn from_xyz(c: char) -> Shape {
        match c {
            'X' => Shape::Rock,
            'Y' => Shape::Paper,
            'Z' => Shape::Scissors,
            _ => panic!("invalid player code: {}", c)
        }
    }
}

enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }

    fn from_shapes(opponenet: &Shape, player: &Shape) -> Self {
        match (opponenet, player) {
            (Shape::Rock, Shape::Paper) => Outcome::Win,
            (Shape::Rock, Shape::Scissors) => Outcome::Lose,
            (Shape::Paper, Shape::Rock) => Outcome::Lose,
            (Shape::Paper, Shape::Scissors) => Outcome::Win,
            (Shape::Scissors, Shape::Rock) => Outcome::Win,
            (Shape::Scissors, Shape::Paper) => Outcome::Lose,
            (_, _) => Outcome::Draw,
        }
    }
}

impl FromXYZ for Outcome {
    fn from_xyz(c: char) -> Outcome {
        match c {
            'X' => Outcome::Lose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => panic!("invalid player code: {}", c)
        }
    }
}

pub fn solve_part_1(input: &str) {
    let guide = strategy_guide(&input);
    
    let mut sum = 0;
    for round in guide {
        let outcome = Outcome::from_shapes(&round.0, &round.1);
        sum += round.1.score() + outcome.score();
    }

    info!("score: {}", sum);
}

pub fn solve_part_2(input: &str) {
    let guide = strategy_guide(&input);
    
    let mut sum = 0;
    for round in guide {
        sum += Shape::from_opponent_and_outcome(&round.0, &round.1).score() + round.1.score();
    }

    info!("score: {}", sum)
}

struct Round{
    abc: char,
    xyz: char,
}

struct ParseRoundError;

impl FromStr for Round {
    type Err = ParseRoundError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars: Vec<char> = s.chars().take(3).collect();
        chars.reverse();
        
        let abc = match chars.pop() {
            Some(c) => c,
            None => return Err(ParseRoundError)
        };
    
        chars.pop();
        
        let xyz = match chars.pop() {
            Some(c) => c,
            None => return Err(ParseRoundError)
        };
    
        Ok(Round{abc: abc, xyz: xyz})
    }
}

fn strategy_guide<T: FromABC, U: FromXYZ> (contents: &str) -> Vec<(T, U)> {
    let mut v = Vec::new();

    for line in contents.lines() {
        if line.is_empty() {
            continue
        }

        match line.parse::<Round>() {
            Ok(round) => v.push((T::from_abc(round.abc), U::from_xyz(round.xyz))),
            Err(_) => (),
        }        
    }

    v
}