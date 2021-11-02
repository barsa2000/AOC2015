use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashMap;

#[aoc_generator(day9)]
fn parse_input(input: &str) -> HashMap<String, HashMap<String, u32>> {
    let mut connections: HashMap<String, HashMap<String, u32>> = HashMap::new();

    input.lines().for_each(|l| {
        let mut split = l.splitn(2, " = ");
        let pair = split.next().unwrap();
        let dist: u32 = split.next().unwrap().parse().unwrap();

        let mut split = pair.splitn(2, " to ");
        let city1 = split.next().unwrap().to_string();
        let city2 = split.next().unwrap().to_string();

        // cities.insert(city1);
        // cities.insert(city2);

        // println!("{} -> {} = {}", city1, city2, dist);

        connections
            .entry(city1.clone())
            .or_insert_with(HashMap::new)
            .insert(city2.clone(), dist);
        connections
            .entry(city2)
            .or_insert_with(HashMap::new)
            .insert(city1, dist);
    });
    // println!("{:#?}", connections);
    connections
    // todo!()
}

#[aoc(day9, part1)]
fn part1(connections: &HashMap<String, HashMap<String, u32>>) -> u32 {
    connections
        .keys()
        .permutations(connections.keys().len())
        .unique()
        .map(|p| {
            p.iter()
                .zip(p.iter().skip(1))
                .map(|(&a, &b)| connections.get(a).unwrap().get(b).unwrap())
                .sum()
        })
        .min()
        .unwrap()
}

#[aoc(day9, part2)]
fn part2(connections: &HashMap<String, HashMap<String, u32>>) -> u32 {
    connections
        .keys()
        .permutations(connections.keys().len())
        .unique()
        .map(|p| {
            p.iter()
                .zip(p.iter().skip(1))
                .map(|(&a, &b)| connections.get(a).unwrap().get(b).unwrap())
                .sum()
        })
        .max()
        .unwrap()
}
// #[aoc(day9, part2)]
// fn part2(connections: &HashMap<String, HashMap<String, u32>>) -> usize {
//     0
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let input = "\
London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141
";
        assert_eq!(part1(&parse_input(input)), 605);
    }
    #[test]
    fn sample2() {
        let input = "\
London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141
";
        assert_eq!(part2(&parse_input(input)), 982);
    }
}
