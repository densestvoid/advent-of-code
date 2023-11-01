use std::{collections::{HashSet, BinaryHeap}, hash::Hash, cmp::Ordering};

use log::{info, debug};

pub fn solve_part_1(input: &str) {
    let positions = parse_chart(input);
    let chart = Chart::new(positions, Elevation::Start);
    info!("steps: {}", chart.navigate());
}

pub fn solve_part_2(input: &str) {
    let positions = parse_chart(input);
    let chart = Chart::new(positions, Elevation::Elevation(0));
    info!("steps: {}", chart.navigate());
}

struct Chart {
    elevations: Vec<Vec<Position>>,
    end: Position,

    step_history: HashSet<Position>,
    next_positions: BinaryHeap<Position>,
}

impl Chart {
    fn new(mut elevations: Vec<Vec<Position>>, starting_elevations: Elevation) -> Self {
        let mut step_history = HashSet::new();
        let mut next_positions = BinaryHeap::new();
        
        let mut end: Option<Position> = None;
        for row in elevations.iter_mut() {
            for pos in row {
                if pos.elevation <= starting_elevations {
                    pos.steps = Some(0);
                    step_history.insert(pos.clone());
                    next_positions.push(pos.clone());
                }
                match pos.elevation {
                    Elevation::End => end = Some(pos.clone()),
                    _ => continue,
                }
            }
        };

        let end = end.expect("no ending position found");

        Chart{
            elevations,
            end,

            step_history,
            next_positions,
        }
    }

    fn navigate(mut self) -> u32 {
        while let Some(curr) = self.next_positions.pop() {
            debug!("{},{}", curr.x, curr.y);

            let next_steps = curr.steps.expect("current position steps not calculated") + 1;
    
            // check down
            if curr.x != 0 {
                match self.elevations.get(curr.x-1) {
                    Some(row) => match row.get(curr.y) {
                        Some(pos) => if curr.elevation.can_navigate(&pos.elevation) {
                            self.check_position(pos.clone(), next_steps);
                        }
                        None => (),
                    }
                    None => (),
                }
            }
    
            // check left
            if curr.y != 0 {
                match self.elevations.get(curr.x) {
                    Some(row) => match row.get(curr.y-1) {
                        Some(pos) => if curr.elevation.can_navigate(&pos.elevation) {
                            self.check_position(pos.clone(), next_steps);
                        }
                        None => (),
                    }
                    None => (),
                }
            }
    
            // check up
            match self.elevations.get(curr.x+1) {
                Some(row) => match row.get(curr.y) {
                    Some(pos) => if curr.elevation.can_navigate(&pos.elevation) {
                        self.check_position(pos.clone(), next_steps);
                    }
                    None => (),
                }
                None => (),
            }
    
            // check right
            match self.elevations.get(curr.x) {
                Some(row) => match row.get(curr.y+1) {
                    Some(pos) => if curr.elevation.can_navigate(&pos.elevation) {
                        self.check_position(pos.clone(), next_steps);
                    }
                    None => (),
                }
                None => (),
            }
        }
    
        self.step_history.get(&self.end).expect("never stepped to end").steps.expect("never updated end steps")
    }

    fn check_position(&mut self, mut pos: Position, new_steps: u32) {
        if let Elevation::End = pos.elevation {
            pos.steps = Some(new_steps);
            self.step_history.insert(pos);
            return
        }

        if !self.step_history.contains(&pos) {
            pos.steps = Some(new_steps);
            self.step_history.insert(pos.clone());
            self.next_positions.push(pos.clone());
        }
    }
}

#[derive(Clone)]
struct Position {
    x: usize,
    y: usize,
    elevation: Elevation,
    steps: Option<u32>,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.steps.partial_cmp(&other.steps) {
            None => None,
            Some(ord) => Some(ord.reverse())
        }
    }
}

impl Eq for Position {}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.elevation.cmp(&other.elevation).reverse() {
            std::cmp::Ordering::Equal => self.steps.cmp(&other.steps).reverse(),
            ord => ord,
        }
    }
}

impl Hash for Position {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

fn parse_chart(input: &str) -> Vec<Vec<Position>> {
    let mut vec = Vec::new();

    for (i, line) in input.lines().enumerate() {
        let mut row = Vec::new();

        for (j, c) in line.chars().enumerate() {
            row.push(Position{x: i, y: j, elevation: Elevation::from(c), steps: None});
        }

        vec.push(row);
    }

    vec
}

#[derive(Clone, Hash)]
enum Elevation {
    Start,
    End,
    Elevation(u8),
}

impl Elevation {
    fn can_navigate(&self, other: &Self) -> bool {
        match (self, other) {
            (_, Self::Start) => true,
            (Self::Start, Self::Elevation(e)) => *e <= 1,
            (Self::Elevation(e1), Self::Elevation(e2)) => *e1 + 1 >= *e2,
            (Self::Elevation(e), Self::End) => e + 1 >= 26,
            (Self::End, _) => true,
            (_, _) => false,
        }
    }
}

impl PartialEq for Elevation {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Start, Self::Start) => true,
            (Self::Elevation(e1), Self::Elevation(e2)) => e1.eq(e2),
            (Self::End, Self::End) => true,
            (_, _) => false,
        }
    }
}

impl PartialOrd for Elevation {
    fn partial_cmp (&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Start, Self::Start) => Some(Ordering::Equal),
            (Self::Start, _) => Some(Ordering::Less),
            (_, Self::Start) => Some(Ordering::Greater),

            (Self::Elevation(e1), Self::Elevation(e2)) => e1.partial_cmp(e2),

            (Self::End, Self::End) => Some(Ordering::Equal),
            (_, Self::End) => Some(Ordering::Less),
            (Self::End, _) => Some(Ordering::Greater),
        }
    }
}

impl Eq for Elevation {}

impl Ord for Elevation {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Start, Self::Start) => Ordering::Equal,
            (Self::Start, _) => Ordering::Less,
            (_, Self::Start) => Ordering::Greater,

            (Self::Elevation(e1), Self::Elevation(e2)) => e1.cmp(e2),

            (Self::End, Self::End) => Ordering::Equal,
            (_, Self::End) => Ordering::Less,
            (Self::End, _) => Ordering::Greater,
        }
    }
}

impl From<char> for Elevation {
    fn from(value: char) -> Self {
        match value {
            'S' => Elevation::Start,
            'E' => Elevation::End,
            _ => Elevation::Elevation(value as u8 - 'a' as u8),
        }
    }
}