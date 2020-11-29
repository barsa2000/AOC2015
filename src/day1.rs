use aoc_runner_derive::{aoc, aoc_generator};
use boolinator::Boolinator;

#[aoc_generator(day1)]
fn parse_input_day1(input: &str) -> Vec<char> {
    input.chars().collect()
}


#[aoc(day1, part1)]
fn part1(directions: &[char]) -> i32 {
    directions.iter().map(|d| match d{
        '(' => 1,
        ')' => -1,
        _ => 0
    }).sum()
}

#[aoc(day1, part2)]
fn part2(directions: &[char]) -> usize {
    directions.iter().scan(0_i32, |state, d| {
        match d {
            '(' => *state += 1,
            ')' => *state -= 1,
            _ => ()
        };
        (!state.is_negative()).as_option()
    }).count() + 1
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_input_day1("((()))")[..]), 0);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&parse_input_day1(")")[..]), 1);
    }

    #[test]
    fn sample3() {
        assert_eq!(part2(&parse_input_day1("())())")[..]), 3);
    }
}
