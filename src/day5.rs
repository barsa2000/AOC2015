use aoc_runner_derive::aoc;

#[aoc(day5, part1)]
fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|l| {
            if l.chars()
                .filter(|&c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'))
                .count()
                < 3
            {
                return false;
            }

            if !l.chars().zip(l.chars().skip(1)).any(|(a, b)| a == b) {
                return false;
            }

            if l.chars()
                .zip(l.chars().skip(1))
                .any(|ab| matches!(ab, ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y')))
            {
                return false;
            }

            true
        })
        .count()
}

#[aoc(day5, part2)]
fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|l| {
            if l.len() < 4 {
                return false;
            }

            let mut found = false;
            for i in 0..l.len() - 3 {
                let pair = &l[i..i + 2];
                if (&l[i + 2..]).contains(pair) {
                    found = true;
                    break;
                }
            }
            if !found {
                return false;
            }

            if !l.chars().zip(l.chars().skip(2)).any(|(a, b)| a == b) {
                return false;
            }

            true
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        assert_eq!(part1("ugknbfddgicrmopn"), 1);
    }
    #[test]
    fn sample2() {
        assert_eq!(part1("aaa"), 1);
    }
    #[test]
    fn sample3() {
        assert_eq!(part1("jchzalrnumimnmhp"), 0);
    }
    #[test]
    fn sample4() {
        assert_eq!(part1("haegwjzuvuyypxyu"), 0);
    }
    #[test]
    fn sample5() {
        assert_eq!(part1("dvszwmarrgswjxmb"), 0);
    }
    #[test]
    fn sample6() {
        assert_eq!(part2("qjhvhtzxzqqjkmpb"), 1);
    }
    #[test]
    fn sample7() {
        assert_eq!(part2("xxyxx"), 1);
    }
    #[test]
    fn sample8() {
        assert_eq!(part2("uurcxstgmygtbstg"), 0);
    }
    #[test]
    fn sample9() {
        assert_eq!(part2("ieodomkazucvgmuy"), 0);
    }
}
