use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::{max, min};
use std::error::Error;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
struct MyBox {
    l: u32,
    w: u32,
    h: u32,
}

#[aoc_generator(day2)]
fn parse_input_day2(input: &str) -> Result<Vec<MyBox>, Box<dyn Error>> {
    input
        .lines()
        .map(|l| {
            let mut measures = l.trim().split('x').map(|m| m.parse::<u32>().unwrap());
            Ok(MyBox {
                l: measures.next().unwrap(),
                w: measures.next().unwrap(),
                h: measures.next().unwrap(),
            })
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(boxes: &[MyBox]) -> u32 {
    boxes
        .iter()
        .map(|b| {
            let s1 = b.l * b.w;
            let s2 = b.w * b.h;
            let s3 = b.h * b.l;
            2 * (s1 + s2 + s3) + min(min(s1, s2), s3)
        })
        .sum()
}

#[aoc(day2, part2)]
fn part2(boxes: &[MyBox]) -> u32 {
    boxes
        .iter()
        .map(|b| 2 * (min(b.l, b.w) + min(max(b.l, b.w), b.h)) + b.l * b.w * b.h)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_input_day2("2x3x4").unwrap()[..]), 58);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&parse_input_day2("2x3x4").unwrap()[..]), 34);
    }

    #[test]
    fn sample3() {
        assert_eq!(part2(&parse_input_day2("1x1x10").unwrap()[..]), 14);
    }
}
