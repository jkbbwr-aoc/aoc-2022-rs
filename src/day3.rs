use aoc_runner_derive::aoc;
use std::collections::HashSet;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_at(line.len() / 2);

            let mut i = 0;
            let left: HashSet<char> = HashSet::from_iter(left.chars());
            let right: HashSet<char> = HashSet::from_iter(right.chars());
            let intersection = left.intersection(&right).collect::<HashSet<_>>();
            for c in intersection {
                if c.is_ascii_uppercase() {
                    i += ((*c as u8) - 38) as i32
                } else {
                    i += ((*c as u8) - 96) as i32
                }
            }
            return i;
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> i32 {
    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunk| {
            let first: HashSet<char> = HashSet::from_iter(chunk[0].chars());
            let second: HashSet<char> = HashSet::from_iter(chunk[1].chars());
            let third: HashSet<char> = HashSet::from_iter(chunk[2].chars());
            let intersection = first
                .intersection(&second)
                .copied()
                .collect::<HashSet<char>>()
                .intersection(&third)
                .copied()
                .collect::<Vec<char>>();
            let c = intersection.first().unwrap();
            if c.is_ascii_uppercase() {
                ((*c as u8) - 38) as i32
            } else {
                ((*c as u8) - 96) as i32
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;
        assert_eq!(part1(&input), 157);
    }

    #[test]
    fn test_part2() {
        let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;
        assert_eq!(part2(&input), 70);
    }
}
