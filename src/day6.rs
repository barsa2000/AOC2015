use aoc_runner_derive::{aoc, aoc_generator};
use std::{error::Error, ops::Range};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
enum InstructionType {
    TurnOn,
    TurnOff,
    Toggle,
}

type Coord = (usize, usize);

type Instruction = (InstructionType, Coord, Coord);

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Result<Vec<Instruction>, Box<dyn Error>> {
    input
        .lines()
        .map(|l| {
            let mut s = l;
            let instruction_type;

            if s.find("turn on").is_some() {
                instruction_type = InstructionType::TurnOn;
                s = s.trim_start_matches("turn on ");
            } else if s.find("turn off").is_some() {
                instruction_type = InstructionType::TurnOff;
                s = s.trim_start_matches("turn off ");
            } else {
                instruction_type = InstructionType::Toggle;
                s = s.trim_start_matches("toggle ");
            }

            let mut split = s.split_whitespace();

            let c1_str = split.next().unwrap();
            let c2_str = split.last().unwrap();

            let mut split = c1_str.split(",");
            let c1 = (
                split.next().unwrap().parse::<usize>().unwrap(),
                split.next().unwrap().parse::<usize>().unwrap(),
            );
            let mut split = c2_str.split(",");
            let c2 = (
                split.next().unwrap().parse::<usize>().unwrap(),
                split.next().unwrap().parse::<usize>().unwrap(),
            );

            Ok((instruction_type, c1, c2))
        })
        .collect()
}

#[aoc(day6, part1)]
fn part1(instructions: &Vec<Instruction>) -> usize {
    let mut grid = [false; 1000 * 1000];

    for instruction in instructions {
        let ins_type = instruction.0;
        let start = instruction.1;
        let end = instruction.2;

        for i in start.0..end.0 + 1 {
            for j in start.1..end.1 + 1 {
                match ins_type {
                    InstructionType::TurnOn => grid[i + j * 1000] = true,
                    InstructionType::TurnOff => grid[i + j * 1000] = false,
                    InstructionType::Toggle => grid[i + j * 1000] = !grid[i + j * 1000],
                }
            }
        }
    }
    grid.iter().filter(|p| **p).count()
}

#[aoc(day6, part2)]
fn part2(instructions: &Vec<Instruction>) -> usize {
    let mut grid = [0_usize; 1000 * 1000];

    for instruction in instructions {
        let ins_type = instruction.0;
        let start = instruction.1;
        let end = instruction.2;

        for i in start.0..end.0 + 1 {
            for j in start.1..end.1 + 1 {
                let val = &mut grid[i + j * 1000];
                match ins_type {
                    InstructionType::TurnOn => *val = val.saturating_add(1),
                    InstructionType::TurnOff => *val = val.saturating_sub(1),
                    InstructionType::Toggle => *val = val.saturating_add(2),
                }
            }
        }
    }
    grid.iter().map(|p| *p).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        assert_eq!(
            part1(&parse_input("turn on 0,0 through 999,999").unwrap()),
            1000 * 1000
        );
    }
}
