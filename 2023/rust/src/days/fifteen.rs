use std::{str::FromStr, fmt::Debug, cmp};

use log::{debug, info};

pub fn solve_part_1(input: &str) {
    let map: Map = match input.parse() {
        Ok(m) => m,
        Err(_) => panic!("failed to parse map"),
    };

    for sensor in &map.sensors {
        debug!("{:?}", sensor);
    }
}

pub fn solve_part_2(_input: &str) {
    
}

struct ParseErr;

struct Map {
    sensors: Vec<Sensor>,
}

impl Map {
    fn beacon_blocked(&self, y: i32) -> u32 {
        let mut min = None;
        let mut max = None;

        for sensor in &self.sensors {
            let (sensor_min, sensor_max) = sensor.beacon_blocked(y);
            
            min = match min {
                None => Some(sensor_min),
                Some(v) => Some(cmp::min(v, sensor_min)),
            };

            max = match max {
                None => Some(sensor_min),
                Some(v) => Some(cmp::min(v, sensor_max)),
            };
        }

        match (min, max) {
            (Some(min), Some(max)) => (max-min) as u32,
            _ => 0,
        }
    }
}

impl FromStr for Map {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sensors = Vec::new();

        for line in s.lines() {
            let sensor = line.parse()?;
            sensors.push(sensor);
        }

        Ok(Map{sensors: sensors})
    }
}

#[derive(Debug)]
struct Sensor {
    location: Point,
    beacon: Point,
}

impl Sensor {
    fn beacon_blocked(&self, y: i32) -> (i32, i32) {
        (0, 0)
    }
}

impl FromStr for Sensor {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (sensor_str, beacon_str) = match s.split_once(": ") {
            Some(split) => split,
            None => return Err(ParseErr),
        };

        let sensor_str = match sensor_str.strip_prefix("Sensor at ") {
            Some(s) => s,
            None => return Err(ParseErr),
        };

        let beacon_str = match beacon_str.strip_prefix("closest beacon is at ") {
            Some(s) => s,
            None => return Err(ParseErr),
        };

        Ok(Sensor{
            location: sensor_str.parse()?,
            beacon: beacon_str.parse()?
        })
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance_to(&self, other: &Self) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl FromStr for Point {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x_str, y_str) = match s.split_once(", ") {
            Some(split) => split,
            None => return Err(ParseErr),
        };

        let x = match x_str.strip_prefix("x=") {
            Some(s) => match s.parse() {
                Ok(d) => d,
                Err(_) => return Err(ParseErr),
            },
            None => return Err(ParseErr),
        };

        let y = match y_str.strip_prefix("y=") {
            Some(s) => match s.parse() {
                Ok(d) => d,
                Err(_) => return Err(ParseErr),
            },
            None => return Err(ParseErr),
        };

        Ok(Point{x, y})
    }
}
