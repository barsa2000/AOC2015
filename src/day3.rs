use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
enum Move {
    Up,
    Down,
    Right,
    Left,
}

#[aoc_generator(day3)]
fn parse_input_day2(input: &str) -> Result<Vec<Move>, Box<dyn Error>> {
    input
        .chars()
        .scan(0, |_, c| match c {
            '^' => Some(Ok(Move::Up)),
            'v' => Some(Ok(Move::Down)),
            '>' => Some(Ok(Move::Right)),
            '<' => Some(Ok(Move::Left)),
            _ => None,
        })
        .collect()
}

#[aoc(day3, part1)]
fn part1(moves: &[Move]) -> usize {
    let mut p = Coord { x: 0, y: 0 };

    let mut visited: HashMap<Coord, bool> = HashMap::new();
    visited.insert(p, true);

    moves.iter().for_each(|m| {
        match m {
            Move::Up => p.y += 1,
            Move::Down => p.y -= 1,
            Move::Right => p.x += 1,
            Move::Left => p.x -= 1,
        }
        visited.insert(p, true);
    });
    visited.keys().count()
}

#[aoc(day3, part2)]
fn part2(moves: &[Move]) -> usize {
    let mut p1 = Coord { x: 0, y: 0 };
    let mut p2 = Coord { x: 0, y: 0 };

    let mut p1_turn = true;

    let mut visited: HashMap<Coord, bool> = HashMap::new();
    visited.insert(p1, true);

    moves.iter().for_each(|m| {
        let mut p = if p1_turn { &mut p1 } else { &mut p2 };
        p1_turn = !p1_turn;
        match m {
            Move::Up => p.y += 1,
            Move::Down => p.y -= 1,
            Move::Right => p.x += 1,
            Move::Left => p.x -= 1,
        }
        visited.insert(*p, true);
    });
    visited.keys().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_input_day2(">").unwrap()[..]), 2);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&parse_input_day2("^v").unwrap()[..]), 3);
    }
}
