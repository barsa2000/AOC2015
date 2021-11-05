use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

#[aoc_generator(day16)]
fn parse_input(input: &str) -> Vec<HashMap<String, u32>> {
    let re = Regex::new(r"Sue \d+: ((?:\w+: \d+,? ?)+)").unwrap();
    let re2 = Regex::new(r"(\w+): (\d+),? ?").unwrap();

    input
        .lines()
        .map(|l| {
            let captures = re.captures(l).unwrap()[1].to_owned();

            let mut attr: HashMap<String, u32> = HashMap::new();

            for c in re2.captures_iter(&captures) {
                let k = c[1].to_owned();
                let v = FromStr::from_str(&c[2]).unwrap();
                attr.insert(k, v);
            }

            attr
        })
        .collect_vec()
}

#[aoc(day16, part1)]
fn part1(input: &[HashMap<String, u32>]) -> usize {
    let mut real_aunt = HashMap::new();
    real_aunt.insert("children".to_owned(), 3);
    real_aunt.insert("cats".to_owned(), 7);
    real_aunt.insert("samoyeds".to_owned(), 2);
    real_aunt.insert("pomeranians".to_owned(), 3);
    real_aunt.insert("akitas".to_owned(), 0);
    real_aunt.insert("vizslas".to_owned(), 0);
    real_aunt.insert("goldfish".to_owned(), 5);
    real_aunt.insert("trees".to_owned(), 3);
    real_aunt.insert("cars".to_owned(), 2);
    real_aunt.insert("perfumes".to_owned(), 1);

    input
        .iter()
        .find_position(|a| a.iter().all(|(k, v)| real_aunt.get(k).unwrap() == v))
        .unwrap()
        .0
        + 1
}

#[aoc(day16, part2)]
fn part2(input: &[HashMap<String, u32>]) -> usize {
    let mut real_aunt_exact = HashMap::new();
    real_aunt_exact.insert("children".to_owned(), 3);
    real_aunt_exact.insert("samoyeds".to_owned(), 2);
    real_aunt_exact.insert("akitas".to_owned(), 0);
    real_aunt_exact.insert("vizslas".to_owned(), 0);
    real_aunt_exact.insert("cars".to_owned(), 2);
    real_aunt_exact.insert("perfumes".to_owned(), 1);

    let mut real_aunt_greater = HashMap::new();
    real_aunt_greater.insert("trees".to_owned(), 3);
    real_aunt_greater.insert("cats".to_owned(), 7);

    let mut real_aunt_fewer = HashMap::new();
    real_aunt_fewer.insert("pomeranians".to_owned(), 3);
    real_aunt_fewer.insert("goldfish".to_owned(), 5);

    input
        .iter()
        .find_position(|a| {
            a.iter().all(|(k, v)| {
                if let Some(n) = real_aunt_exact.get(k) {
                    if n == v {
                        return true;
                    }
                }
                if let Some(n) = real_aunt_greater.get(k) {
                    if n < v {
                        return true;
                    }
                }
                if let Some(n) = real_aunt_fewer.get(k) {
                    if n > v {
                        return true;
                    }
                }
                false
            })
        })
        .unwrap()
        .0
        + 1
}
