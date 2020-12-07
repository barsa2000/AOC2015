use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Ops {
    And,
    Or,
    Lshift,
    Rshift,
    Not,
    Assign,
}

#[aoc_generator(day7)]
fn parse_input(input: &str) -> HashMap<String, (String, Ops, String)> {
    input
        .lines()
        .map(|l| {
            let mut split = l.split(" -> ");
            let val = split.next().unwrap();
            let key = split.next().unwrap();

            let split: Vec<&str> = val.split_whitespace().collect();
            let len = split.len();
            (
                key.to_string(),
                match len {
                    1 => (split[0].to_string(), Ops::Assign, "".to_string()),
                    2 => (split[1].to_string(), Ops::Not, "".to_string()),
                    3 => (
                        split[0].to_string(),
                        match split[1] {
                            "AND" => Ops::And,
                            "OR" => Ops::Or,
                            "LSHIFT" => Ops::Lshift,
                            "RSHIFT" => Ops::Rshift,
                            _ => unreachable!(),
                        },
                        split[2].to_string(),
                    ),
                    _ => unreachable!(),
                },
            )
        })
        .collect()
}

#[aoc(day7, part1)]
fn part1(input: &HashMap<String, (String, Ops, String)>) -> u16 {
    calc_val(input, &mut HashMap::new(), "a")
}

#[aoc(day7, part2)]
fn part2(input: &HashMap<String, (String, Ops, String)>) -> u16 {
    let mut res = HashMap::new();
    let b = calc_val(input, &mut res, "a");
    let mut res = HashMap::new();
    res.insert("b".to_string(), b);
    calc_val(input, &mut res, "a")
}

fn calc(
    map: &HashMap<String, (String, Ops, String)>,
    results: &mut HashMap<String, u16>,
    value: (&str, Ops, &str),
) -> u16 {
    match value.1 {
        Ops::Assign => calc_val(map, results, value.0),
        Ops::Not => !calc_val(map, results, value.0),
        Ops::And => calc_val(map, results, value.0) & calc_val(map, results, value.2),
        Ops::Or => calc_val(map, results, value.0) | calc_val(map, results, value.2),
        Ops::Rshift => calc_val(map, results, value.0) >> calc_val(map, results, value.2),
        Ops::Lshift => calc_val(map, results, value.0) << calc_val(map, results, value.2),
    }
}

fn calc_val(
    map: &HashMap<String, (String, Ops, String)>,
    results: &mut HashMap<String, u16>,
    value: &str,
) -> u16 {
    if let Ok(v) = value.parse::<u16>() {
        v
    } else if let Some(res) = results.get(value) {
        *res
    } else {
        let val = map.get(value).unwrap();
        let res = calc(map, results, (val.0.as_str(), val.1, val.2.as_str()));
        results.insert(value.to_string(), res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let input = "\
1 -> a
123 -> x
456 -> y
NOT 5 -> z
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";
        assert_eq!(part1(&parse_input(input)), 1);
    }
}
