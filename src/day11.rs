use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day11)]
fn parse_input(input: &str) -> Vec<char> {
    input.chars().rev().collect()
}

fn is_valid(chars: &[char]) -> bool {
    let mut found_increasing_straight = false;
    let mut was_increasing = false;
    let mut last_char = '\0';

    let mut found_two_pairs = false;
    let mut first_pair_char = '\0';

    for &c in chars {
        if c == 'i' || c == 'o' || c == 'l' {
            return false;
        }
        if !found_increasing_straight {
            if c as u8 - last_char as u8 == 1 {
                if was_increasing {
                    found_increasing_straight = true;
                } else {
                    was_increasing = true;
                }
            } else if was_increasing {
                was_increasing = false
            }
        }
        if !found_two_pairs && c != first_pair_char && c == last_char {
            if first_pair_char == '\0' {
                first_pair_char = c;
            } else {
                found_two_pairs = true;
            }
        }
        last_char = c;
    }

    found_increasing_straight && found_two_pairs
}

fn inc(s: &mut Vec<char>) {
    for c in s {
        if *c == 'z' {
            *c = 'a';
            continue;
        }
        *c = (*c as u8 + 1) as char;
        return;
    }
}

fn find_next_valid(chars: &[char]) -> String {
    let mut chars = chars.to_owned();

    loop {
        inc(&mut chars);

        if is_valid(&chars) {
            break;
        }
    }

    chars.iter().collect()
}

#[aoc(day11, part1)]
fn part1(chars: &[char]) -> String {
    find_next_valid(chars).chars().rev().collect()
}

#[aoc(day11, part2)]
fn part2(chars: &[char]) -> String {
    find_next_valid(&find_next_valid(chars).chars().collect_vec())
        .chars()
        .rev()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let chars = "hijklmmn";
        assert!(!is_valid(&parse_input(chars)));
    }

    #[test]
    fn sample2() {
        let chars = "abbceffg";
        assert!(!is_valid(&parse_input(chars)));
    }

    #[test]
    fn sample3() {
        let chars = "abbcegjk";
        assert!(!is_valid(&parse_input(chars)));
    }

    #[test]
    fn sample4() {
        let chars = "abcdefgh";
        assert_eq!(part1(&parse_input(chars)), "abcdffaa");
    }
}
