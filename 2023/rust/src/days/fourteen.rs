use std::{collections::VecDeque, str::FromStr, fmt::{Debug, Write}, cmp};

use log::info;

pub fn solve_part_1(input: &str) {
    let formations = parse_formations(input);
    let mut cave = Cave::new(formations);

    let sand = cave.fill_abyss();

    info!("amount of sand: {}", sand);
}

pub fn solve_part_2(input: &str) {
    let formations = parse_formations(input);
    let mut cave = Cave::new(formations);

    let sand = cave.fill_floor();

    info!("amount of sand: {}", sand);
}

struct ParseError;

struct Cave{
    left: usize,
    right: usize,
    depth: usize,
    formation: Vec<VecDeque<State>>,
}

impl Cave {
    fn new(formations: Vec<Formation>) -> Self {
        let mut cave = Self {
            left: 500,
            right: 500,
            depth: 0,
            formation: vec![VecDeque::from(vec![State::Source])],
        };

        for formation in formations {
            cave.add_formation(formation);
        }

        cave.add_columns_left(1);
        cave.add_columns_right(1);

        cave
    }

    fn add_formation(&mut self, formation: Formation) {
        for (start, end) in formation.points[0..formation.points.len()-1].iter().zip(formation.points[1..formation.points.len()].iter()) {
            self.adjust_for_point(start);
            self.adjust_for_point(end);

            if start.x == end.x {
                let min = cmp::min(start.y, end.y);
                let max = cmp::max(start.y, end.y);
                for y in min..=max {
                    self.fill_rock(Point{x: start.x, y: y});
                }
            } else if start.y == end.y {
                let min = cmp::min(start.x, end.x);
                let max = cmp::max(start.x, end.x);
                for x in min..=max {
                    self.fill_rock(Point{x: x, y: start.y});
                }
            }
        }
    }

    fn fill_rock(&mut self, point: Point) {
        let x = point.x-self.left;
        let y = point.y;

        self.formation[y][x] = State::Rock;
    }

    fn adjust_for_point(&mut self, point: &Point) {
        if point.x < self.left {
            self.add_columns_left(self.left-point.x);
        }
        if point.x > self.right {
            self.add_columns_right(point.x-self.right);
        }
        if point.y > self.depth {
            self.add_row(point.y-self.depth);
        }
    }

    fn add_columns_left(&mut self, num: usize) {
        for row in &mut self.formation {
            for _ in 0..num {
                row.push_front(State::Air);
            }
        }
        self.left -= num;
    }

    fn add_columns_right(&mut self, num: usize) {
        for row in &mut self.formation {
            for _ in 0..num {
                row.push_back(State::Air);
            }
        }
        self.right += num;
    }

    fn add_row(&mut self, num: usize) {
        for _ in 0..num {
            self.formation.push(VecDeque::from(vec![State::Air; self.right - self.left + 1]));
        }
        self.depth += num;
    }

    fn add_floor(&mut self) {
        self.add_row(1);
        
        self.formation.push(VecDeque::from(vec![State::Rock; self.right - self.left + 1]));

        self.depth += 1;
    }

    fn fill_abyss(&mut self) -> u32 {
        let mut total_sand = 0;

        loop {
            let mut x = 500-self.left;
            let mut y = 0;
            loop {
                match self.formation.get_mut(y+1) {
                    None => {
                        return total_sand;
                    }
                    Some(row) => {
                        // try straight down
                        if row[x].open() {
                            y += 1;
                            continue;
                        }

                        // try left down
                        if x != 0 && row[x-1].open() {
                            y += 1;
                            x -= 1;
                            continue;
                        }

                        // try right down
                        if x != row.len()-1 && row[x+1].open() {
                            y += 1;
                            x += 1;
                            continue;
                        }

                        // sand has stopped falling
                        self.formation[y][x] = State::Sand;
                        total_sand += 1;
                        break;
                    }
                }
            }
        }
    }

    fn fill_floor(&mut self) -> u32 {
        self.add_columns_left(self.depth);
        self.add_columns_right(self.depth);
        self.add_floor();

        let mut total_sand = 0;

        loop {
            let mut x = 500-self.left;
            let mut y = 0;

            if !self.formation[y][x].open() {
                return total_sand;
            }            
            loop {
                match self.formation.get_mut(y+1) {
                    None => {
                        return total_sand;
                    }
                    Some(row) => {
                        // try straight down
                        if row[x].open() {
                            y += 1;
                            continue;
                        }

                        // try left down
                        if x != 0 && row[x-1].open() {
                            y += 1;
                            x -= 1;
                            continue;
                        }

                        // try right down
                        if x != row.len()-1 && row[x+1].open() {
                            y += 1;
                            x += 1;
                            continue;
                        }

                        // sand has stopped falling
                        self.formation[y][x] = State::Sand;
                        total_sand += 1;
                        break;
                    }
                }
            }
        }
    }
}

fn parse_formations(input: &str) -> Vec<Formation> {
    let mut vec = Vec::new();

    for line in input.lines() {
        match line.parse() {
            Ok(f) => vec.push(f),
            Err(_) => continue,
        }
    }

    vec
}

struct Formation {
    points: Vec<Point>,
}

impl FromStr for Formation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut points = Vec::new();

        for point in s.split(" -> ") {
            match point.parse() {
                Ok(point) => points.push(point),
                Err(_) => return Err(ParseError),
            }
        }

        Ok(Formation{points: points})
    }
}

struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = match s.split_once(",") {
            Some(p) => p,
            None => return Err(ParseError),
        };

        let x = match x.parse() {
            Ok(d) => d,
            Err(_) => return Err(ParseError),
        };

        let y = match y.parse() {
            Ok(d) => d,
            Err(_) => return Err(ParseError),
        };

        Ok(Point{x: x, y: y})
    }
}

#[derive(Clone)]
enum State{
    Source,
    Air,
    Rock,
    Sand,
}

impl State {
    fn open(&self) -> bool {
        match self {
            Self::Source => true,
            Self::Air => true,
            _ => false,
        }
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Source => f.write_char('+'),
            Self::Air => f.write_char('.'),
            Self::Rock => f.write_char('#'),
            Self::Sand => f.write_char('o'),
        }
    }
}