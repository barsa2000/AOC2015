use aoc_runner_derive::{aoc, aoc_generator};
use json::JsonValue;

#[aoc_generator(day12)]
fn parse_input_day1(input: &str) -> JsonValue {
    json::parse(input).unwrap()
}

fn sum_json(json: &JsonValue) -> i64 {
    // println!("{:#?}", json);

    if json.is_array() {
        return json.members().map(|m| sum_json(m)).sum();
    } else if json.is_number() {
        return json.as_i64().unwrap();
    } else if json.is_object() {
        return json.entries().map(|(_, e)| sum_json(e)).sum();
    }
    0
}

fn sum_json2(json: &JsonValue) -> i64 {
    // println!("{:#?}", json);

    if json.is_array() {
        return json.members().map(|m| sum_json2(m)).sum();
    } else if json.is_number() {
        return json.as_i64().unwrap();
    } else if json.is_object() {
        if json
            .entries()
            .any(|(_, e)| e.is_string() && e.to_string().eq("red"))
        {
            return 0;
        }
        return json.entries().map(|(_, e)| sum_json2(e)).sum();
    }
    0
}

#[aoc(day12, part1)]
fn part1(json: &JsonValue) -> i64 {
    sum_json(json)
}

#[aoc(day12, part2)]
fn part2(json: &JsonValue) -> i64 {
    sum_json2(json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let parsed = json::parse(
            r#"
{
    "a": {
        "b":10
    },
    "c": 10,
    "d": [1,2,3]
}

"#,
        )
        .unwrap();
        assert_eq!(part1(&parsed), 26);
    }
    #[test]
    fn sample2() {
        let parsed = json::parse(r#"[[[3]]]"#).unwrap();
        assert_eq!(part1(&parsed), 3);
    }
    #[test]
    fn sample3() {
        let parsed = json::parse(r#"[1,{"c":"red","b":2},3]"#).unwrap();
        assert_eq!(part2(&parsed), 4);
    }
}
