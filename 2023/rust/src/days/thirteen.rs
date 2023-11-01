use std::{str::FromStr, collections::VecDeque};

use log::{info, debug};

pub fn solve_part_1(input: &str) {
    let pairs = parse_packetdata_set(input);

    let mut sum = 0;
    for (i, pair) in pairs.iter().enumerate() {
        debug!("{:?}", pair);
        let in_order = pair.in_order();
        debug!("{}: {}", i+1, in_order);
        if in_order {
            sum += i+1;
        }
    }

    info!("sum: {}", sum)
}

pub fn solve_part_2(input: &str) {
    let pd2 = PacketData::List(vec![PacketData::Int(2)]);
    let pd6 = PacketData::List(vec![PacketData::Int(6)]);

    let mut packets = parse_packetdata(input);
    packets.push(pd2.clone());
    packets.push(pd6.clone());
    packets.sort();

    let idx2 = match packets.binary_search(&pd2) {
        Ok(v) => v,
        Err(_) => panic!("should be found")
    };
    let idx6 = match packets.binary_search(&pd6) {
        Ok(v) => v,
        Err(_) => panic!("should be found")
    };

    info!("decoder key: {}", (idx2+1) * (idx6+1))
}

fn parse_packetdata_set(input: &str) -> Vec<PacketPair> {
    let mut vec = Vec::new();

    for pair in input.split("\n\n") {
        let (first, second) = match pair.split_once("\n") {
            Some(split) => split,
            None => continue,
        };

        let first = match PacketData::from_str(first) {
            Ok(pd) => pd,
            Err(_) => continue,
        };

        let second = match PacketData::from_str(second) {
            Ok(pd) => pd,
            Err(_) => continue,
        };
        
        vec.push(PacketPair { first: first, second: second });
    }

    vec
}

fn parse_packetdata(input: &str) -> Vec<PacketData> {
    let mut vec = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let pd = match PacketData::from_str(line) {
            Ok(pd) => pd,
            Err(_) => continue,
        };
        
        vec.push(pd);
    }

    vec
}

#[derive(Debug)]
struct PacketPair {
    first: PacketData,
    second: PacketData,
}

impl PacketPair {
    fn in_order(&self) -> bool {
        self.first <= self.second
    }
}

#[derive(Clone, Debug)]
enum PacketData {
    Int(u32),
    List(Vec<Self>),
}

struct ParsePacketDataError;

impl PacketData {
    fn subparse(chars: &mut VecDeque<char>) -> Result<Self, ParsePacketDataError> {
        let mut data = Vec::new();

        while let Some(c) = chars.pop_front() {
            match c {
                ',' => continue,
                '[' => match PacketData::subparse(chars) {
                    Ok(pd) => data.push(pd),
                    Err(e) => return Err(e),
                }
                ']' => break,
                _ => {
                    let mut digits = String::from(c);
                    while let Some(c) = chars.front() {
                        if !c.is_digit(10) {
                            break
                        }
                        if let Some(c) = chars.pop_front() {
                            digits.push(c);
                        }
                    }

                    if let Ok(d) = digits.parse() {
                        data.push(PacketData::Int(d));
                    }
                }
            }
        }

        Ok(PacketData::List(data))
    }
}

impl PartialEq for PacketData {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(i1), Self::Int(i2)) => i1.eq(i2),
            (Self::Int(_), Self::List(_)) => PacketData::List(vec![self.clone()]).eq(other),
            (Self::List(_), Self::Int(_)) => self.eq(&PacketData::List(vec![other.clone()])),
            (Self::List(l1), Self::List(l2)) => l1.eq(l2),
        }
    }
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Int(i1), Self::Int(i2)) => i1.partial_cmp(i2),
            (Self::Int(_), Self::List(_)) => PacketData::List(vec![self.clone()]).partial_cmp(other),
            (Self::List(_), Self::Int(_)) => self.partial_cmp(&PacketData::List(vec![other.clone()])),
            (Self::List(l1), Self::List(l2)) => l1.partial_cmp(l2),
        }
    }
}

impl Eq for PacketData {}

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Int(i1), Self::Int(i2)) => i1.cmp(i2),
            (Self::Int(_), Self::List(_)) => PacketData::List(vec![self.clone()]).cmp(other),
            (Self::List(_), Self::Int(_)) => self.cmp(&PacketData::List(vec![other.clone()])),
            (Self::List(l1), Self::List(l2)) => l1.cmp(l2),
        }
    }
}

impl FromStr for PacketData {
    type Err = ParsePacketDataError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.strip_prefix("[") {
            Some(s) => match s.strip_suffix("]") {
                Some(s) => PacketData::subparse(&mut s.chars().collect()),
                None => Err(ParsePacketDataError),
            }
            None => Err(ParsePacketDataError),

        }
    }
}