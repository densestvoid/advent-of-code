use std::str::FromStr;

use log::info;

pub fn solve_part_1(input: &str) {
    let mut cpu = CPU{register_x: 1, cycle: 0};

    let mut signal_strength = 0;
    let mut next_cycle = 20;
    let instructions = parse_instructions(input);
    for ins in instructions {
        let out = cpu.process(ins, next_cycle);
        if out > 0 {
            next_cycle += 40;
            signal_strength += out;
        }
    }

    info!("signal strength: {}", signal_strength);
}

pub fn solve_part_2(input: &str) {
    let mut screen = Screen{register_x: 1, cycle: 0, lines: [['.'; 40]; 6]};

    let instructions = parse_instructions(input);
    for ins in instructions {
        screen.process(ins);
    }

    screen.display();
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let mut vec = Vec::new();

    for line in input.lines() {
        match line.parse() {
            Ok(ins) => vec.push(ins),
            Err(_) => continue,
        }
    }

    vec
}

struct Screen {
    register_x: i32,
    cycle: u16,
    lines: [[char; 40]; 6],
}

impl Screen {
    fn process(&mut self, ins: Instruction) {
        match ins {
            Instruction::Addx(val) => {
                let line = self.cycle as usize / 40;
                let c = self.cycle as usize % 40;

                if self.register_x-1 <= c as i32 && c as i32 <= self.register_x+1 {
                    self.lines[line][c] = '#';
                }

                self.cycle += 1;

                let line = self.cycle as usize / 40;
                let c = self.cycle as usize % 40;
                if self.register_x-1 <= (c) as i32 && (c) as i32 <= self.register_x+1 {
                    self.lines[line][c] = '#';
                }
                
                self.cycle += 1;
                self.register_x += val;
            }
            Instruction::Noop => {
                let line = self.cycle as usize / 40;
                let c = self.cycle as usize % 40;
                if self.register_x-1 <= c as i32 && c as i32 <= self.register_x+1 {
                    self.lines[line][c] = '#';
                }

                self.cycle += 1;
            },
        }
    }

    fn display(&self) {
        for line in self.lines {
            info!("{}", String::from_iter(line.iter()));
        }
    }
}

struct CPU {
    register_x: i32,
    cycle: u16,
}

impl CPU {
    fn process(&mut self, ins: Instruction, cycle_report: u16) -> i32 {
        match ins {
            Instruction::Addx(val) => {
                let mut ret = 0;

                self.cycle += 2;
                if self.cycle >= cycle_report {
                    ret = self.register_x * cycle_report as i32;
                }
                self.register_x += val;

                ret
            }
            Instruction::Noop => {
                self.cycle += 1;
                if self.cycle == cycle_report {
                    return self.register_x * self.cycle as i32;
                }

                0
            },
        }
    }
}

enum Instruction {
    Addx(i32),
    Noop,
}

struct ParseInstructionError;

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(" ") {
            Some((ins, value)) => {
                match ins {
                    "addx" => match value.parse() {
                        Ok(value) => Ok(Instruction::Addx(value)),
                        Err(_) => Err(ParseInstructionError),
                    }
                    _ => Err(ParseInstructionError),
                }
            }
            None => match s {
                "noop" => Ok(Instruction::Noop),
                _ => Err(ParseInstructionError),
            }
        }
    }
}