use aoc_runner_derive::aoc;
use regex::Regex;

#[aoc(day8, part1)]
fn part1(input: &str) -> usize {
    let regex = Regex::new(r#"[^\\"]|\\"|\\x[\da-f]{2}|\\\\"#).unwrap();
    input
        .lines()
        .map(|l| l.len() - regex.find_iter(l).count())
        .sum()
}

#[aoc(day8, part2)]
fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|l| l.escape_default().to_string().len() + 2 - l.len())
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let input = "\
\"\"
\"abc\"
\"aaa\\\"aaa\"
\"\\x27\"
";
        assert_eq!(part1(input), 12);
    }

    #[test]
    fn sample2() {
        let input = "\
\"\"
\"abc\"
\"aaa\\\"aaa\"
\"\\x27\"
";
        assert_eq!(part2(input), 19);
    }
}
