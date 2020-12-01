use aoc_runner_derive::aoc;

use crypto::digest::Digest;
use crypto::md5::Md5;

#[aoc(day4, part1)]
fn part1(key: &str) -> u64 {
    let mut hasher = Md5::new();

    hasher.input_str(key);

    for i in 0..u64::MAX {
        let mut hasher = hasher;
        hasher.input_str(&i.to_string());

        let mut res = [0_u8; 16];
        hasher.result(&mut res);

        if res[0..2] == [0; 2] && res[2] & 0xF0 == 0 {
            return i;
        }
    }
    return 0;
    // hasher.input(key);
}

#[aoc(day4, part2)]
fn part2(key: &str) -> u64 {
    let mut hasher = Md5::new();

    hasher.input_str(key);

    for i in 0..u64::MAX {
        let mut hasher = hasher;
        hasher.input_str(&i.to_string());

        let mut res = [0_u8; 16];
        hasher.result(&mut res);

        if res[0..3] == [0; 3] {
            return i;
        }
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        assert_eq!(part1("abcdef"), 609043);
    }
}
