use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
#[aoc_generator(day13)]
fn parse_input(input: &str) -> (HashSet<String>, HashMap<(String, String), i32>) {
    let mut connections: HashMap<(String, String), i32> = HashMap::new();
    let mut names = HashSet::new();

    let re = Regex::new(r"(\w*) would (gain|lose) (\d+) happiness units by sitting next to (\w*).")
        .unwrap();
    input.lines().for_each(|l| {
        let captures = re.captures(l).unwrap();

        let name1 = captures.get(1).unwrap().as_str().to_string();
        let value: i32 = match captures.get(2).unwrap().as_str() {
            "gain" => FromStr::from_str(captures.get(3).unwrap().as_str()).unwrap(),
            "lose" => -<i32 as FromStr>::from_str(captures.get(3).unwrap().as_str()).unwrap(),
            _ => unreachable!(),
        };
        let name2 = captures.get(4).unwrap().as_str().to_string();

        names.insert(name1.clone());
        let k1 = (name1.clone(), name2.clone());
        let k2 = (name2, name1);

        *connections.entry(k1).or_insert(0) += value;
        *connections.entry(k2).or_insert(0) += value;
    });
    (names, connections)
}

#[aoc(day13, part1)]
fn part1(input: &(HashSet<String>, HashMap<(String, String), i32>)) -> i32 {
    let mut names = input.0.clone();
    let connections = input.1.clone();

    let start = names.to_owned().iter().last().unwrap().to_owned();
    names.remove(&start);

    names
        .iter()
        .permutations(names.len())
        .unique()
        .map(|p| {
            let first = p.first().unwrap();
            let last = p.last().unwrap();
            p.iter()
                .zip(p.iter().skip(1))
                .map(|(&a, &b)| connections.get(&(a.to_string(), b.to_string())).unwrap())
                .sum::<i32>()
                + connections
                    .get(&(start.clone(), first.to_string()))
                    .unwrap()
                + connections.get(&(start.clone(), last.to_string())).unwrap()
        })
        .max()
        .unwrap()
}

#[aoc(day13, part2)]
fn part2(input: &(HashSet<String>, HashMap<(String, String), i32>)) -> i32 {
    let mut names = input.0.clone();
    let mut connections = input.1.clone();

    names.iter().for_each(|n| {
        connections.insert(("me".to_string(), n.to_string()), 0);
        connections.insert((n.to_string(), "me".to_string()), 0);
    });
    names.insert("me".to_string());

    let start = names.to_owned().iter().last().unwrap().to_owned();
    names.remove(&start);

    names
        .iter()
        .permutations(names.len())
        .unique()
        .map(|p| {
            let first = p.first().unwrap();
            let last = p.last().unwrap();
            p.iter()
                .zip(p.iter().skip(1))
                .map(|(&a, &b)| connections.get(&(a.to_string(), b.to_string())).unwrap())
                .sum::<i32>()
                + connections
                    .get(&(start.clone(), first.to_string()))
                    .unwrap()
                + connections.get(&(start.clone(), last.to_string())).unwrap()
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
Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.
";
        assert_eq!(part1(&parse_input(input)), 330);
    }
}
