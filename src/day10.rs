use aoc_runner_derive::{aoc, aoc_generator};
use crypto::buffer::ReadBuffer;
use itertools::Itertools;

#[aoc_generator(day10)]
pub fn parse_input_as_numbers(input: &str) -> Vec<u8> {
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

fn look_and_say(input: &Vec<u8>) -> Vec<u8> {
    let mut res = vec![];
    let mut iter = input.iter().peekable();

    while let Some(n) = iter.next() {
        let mut count = 1;
        while let Some(next) = iter.peek() {
            if *next != n {
                break;
            }

            iter.next();
            count += 1;
        }
        res.push(count);
        res.push(*n);
    }
    res
}

#[aoc(day10, part1)]
fn part1(input: &Vec<u8>) -> usize {
    let mut v = input.to_owned();
    for _ in 0..40 {
        v = look_and_say(&v);
    }
    v.len()
}

#[aoc(day10, part2)]
fn part2(input: &Vec<u8>) -> usize {
    let mut v = input.to_owned();
    for _ in 0..50 {
        v = look_and_say(&v);
    }
    v.len()
}
