use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use regex::Regex;
use std::cmp;
use std::collections::HashMap;
use std::str::FromStr;

#[aoc_generator(day14)]
fn parse_input(input: &str) -> HashMap<String, ((u64, u64), u64)> {
    let re = Regex::new(
        r"(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.",
    )
    .unwrap();

    let mut ret = HashMap::new();

    input.lines().for_each(|l| {
        let captures = re.captures(l).unwrap();

        let name = captures.get(1).unwrap().as_str().to_string();
        let v = <u64 as FromStr>::from_str(captures.get(2).unwrap().as_str()).unwrap();
        let tr = <u64 as FromStr>::from_str(captures.get(3).unwrap().as_str()).unwrap();
        let ts = <u64 as FromStr>::from_str(captures.get(4).unwrap().as_str()).unwrap();

        ret.insert(name, ((tr, v), ts));
    });

    ret
}

#[aoc(day14, part1)]
fn part1(input: &HashMap<String, ((u64, u64), u64)>) -> u64 {
    let f = 2503;
    input
        .iter()
        .map(|(_, value)| {
            let tr = value.0 .0;
            let v = value.0 .1;
            let ts = value.1;

            (f / (tr + ts)) * tr * v + cmp::min(tr, f % (tr + ts)) * v
        })
        .max()
        .unwrap()
}

#[aoc(day14, part2)]
fn part2(input: &HashMap<String, ((u64, u64), u64)>) -> u64 {
    let max_f = 2503;
    let mut points = HashMap::new();
    input.keys().for_each(|k| {
        points.insert(k, 0);
    });
    for f in 1..=max_f {
        let mut max = 0;
        let times = input
            .iter()
            .map(|(k, value)| {
                let tr = value.0 .0;
                let v = value.0 .1;
                let ts = value.1;

                let out = (f / (tr + ts)) * tr * v + cmp::min(tr, f % (tr + ts)) * v;
                max = cmp::max(max, out);
                (k, out)
            })
            .collect_vec();

        times.iter().filter(|(_, v)| *v == max).for_each(|(k, _)| {
            *points.get_mut(k).unwrap() += 1;
        });
    }

    *points.values().max().unwrap()
}
