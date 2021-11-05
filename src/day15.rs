use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use regex::Regex;
use std::cmp;
use std::collections::HashMap;
use std::str::FromStr;

#[aoc_generator(day15)]
fn parse_input(input: &str) -> HashMap<String, (i64, i64, i64, i64, i64)> {
    let re = Regex::new(
        r"(.*): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)",
    )
    .unwrap();

    let mut ret = HashMap::new();

    input.lines().for_each(|l| {
        let captures = re.captures(l).unwrap();

        let name = captures.get(1).unwrap().as_str().to_string();
        let cap = <i64 as FromStr>::from_str(captures.get(2).unwrap().as_str()).unwrap();
        let dur = <i64 as FromStr>::from_str(captures.get(3).unwrap().as_str()).unwrap();
        let fla = <i64 as FromStr>::from_str(captures.get(4).unwrap().as_str()).unwrap();
        let tex = <i64 as FromStr>::from_str(captures.get(5).unwrap().as_str()).unwrap();
        let cal = <i64 as FromStr>::from_str(captures.get(6).unwrap().as_str()).unwrap();

        ret.insert(name, (cap, dur, fla, tex, cal));
    });

    ret
}

#[aoc(day15, part1)]
fn part1(input: &HashMap<String, (i64, i64, i64, i64, i64)>) -> u64 {
    (0..=100)
        .permutations(input.len() - 1)
        .filter(|p| p.iter().sum::<i64>() < 100)
        .map(|p| {
            let last: i64 = 100 - p.iter().sum::<i64>();
            let cap: i64 = input
                .values()
                .zip(p.iter())
                .map(|((v, _, _, _, _), w)| *v * w)
                .sum::<i64>()
                + input.values().last().unwrap().0 * last;
            if cap <= 0 {
                return 0;
            }
            let dur: i64 = input
                .values()
                .zip(p.iter())
                .map(|((_, v, _, _, _), w)| *v * w)
                .sum::<i64>()
                + input.values().last().unwrap().1 * last;
            if dur <= 0 {
                return 0;
            }
            let fla: i64 = input
                .values()
                .zip(p.iter())
                .map(|((_, _, v, _, _), w)| *v * w)
                .sum::<i64>()
                + input.values().last().unwrap().2 * last;
            if fla <= 0 {
                return 0;
            }
            let tex: i64 = input
                .values()
                .zip(p.iter())
                .map(|((_, _, _, v, _), w)| *v * w)
                .sum::<i64>()
                + input.values().last().unwrap().3 * last;
            if tex <= 0 {
                return 0;
            }
            cap as u64 * dur as u64 * fla as u64 * tex as u64
        })
        .max()
        .unwrap()
}
#[aoc(day15, part2)]
fn part2(input: &HashMap<String, (i64, i64, i64, i64, i64)>) -> u64 {
    (0..=100)
        .permutations(input.len() - 1)
        .filter(|p| p.iter().sum::<i64>() < 100)
        .map(|p| {
            let last: i64 = 100 - p.iter().sum::<i64>();
            let cals: i64 = input
                .values()
                .zip(p.iter())
                .map(|((_, _, _, _, v), w)| *v * w)
                .sum::<i64>()
                + input.values().last().unwrap().4 * last;

            if cals != 500 {
                return 0;
            }

            let cap: i64 = input
                .values()
                .zip(p.iter())
                .map(|((v, _, _, _, _), w)| *v * w)
                .sum::<i64>()
                + input.values().last().unwrap().0 * last;
            if cap <= 0 {
                return 0;
            }
            let dur: i64 = input
                .values()
                .zip(p.iter())
                .map(|((_, v, _, _, _), w)| *v * w)
                .sum::<i64>()
                + input.values().last().unwrap().1 * last;
            if dur <= 0 {
                return 0;
            }
            let fla: i64 = input
                .values()
                .zip(p.iter())
                .map(|((_, _, v, _, _), w)| *v * w)
                .sum::<i64>()
                + input.values().last().unwrap().2 * last;
            if fla <= 0 {
                return 0;
            }
            let tex: i64 = input
                .values()
                .zip(p.iter())
                .map(|((_, _, _, v, _), w)| *v * w)
                .sum::<i64>()
                + input.values().last().unwrap().3 * last;
            if tex <= 0 {
                return 0;
            }
            cap as u64 * dur as u64 * fla as u64 * tex as u64
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let input = "\
Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
";
        assert_eq!(part1(&parse_input(input)), 62842880);
    }
    #[test]
    fn sample2() {
        let input = "\
Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
";
        assert_eq!(part2(&parse_input(input)), 57600000);
    }
}
