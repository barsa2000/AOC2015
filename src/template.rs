use aoc_runner_derive::{aoc, aoc_generator};
use std::error::Error;

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Result<Vec<MyBox>, Box<dyn Error>> {

}

#[aoc(day2, part1)]
fn part1(boxes: &[MyBox]) -> u32 {

}

#[aoc(day2, part2)]
fn part2(boxes: &[MyBox]) -> u32 {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_input("").unwrap()[..]), 58);
    }

}
