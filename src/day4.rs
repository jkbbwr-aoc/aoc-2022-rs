use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::ops::RangeInclusive;

#[aoc_generator(day4)]
pub fn generator(input: &str) -> Vec<(RangeInclusive<i32>, RangeInclusive<i32>)> {
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            (
                caps.get(1).unwrap().as_str().parse::<i32>().unwrap()
                    ..=caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                caps.get(3).unwrap().as_str().parse::<i32>().unwrap()
                    ..=caps.get(4).unwrap().as_str().parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<_>>()
}

#[aoc(day4, part1)]
pub fn part1(input: &[(RangeInclusive<i32>, RangeInclusive<i32>)]) -> i32 {
    input
        .iter()
        .filter(|(left, right)| {
            left.clone().into_iter().all(|i| right.contains(&i))
                || right.clone().into_iter().all(|i| left.contains(&i))
        })
        .count()
        .try_into()
        .unwrap()
}

#[aoc(day4, part2)]
pub fn part2(input: &[(RangeInclusive<i32>, RangeInclusive<i32>)]) -> i32 {
    input
        .iter()
        .filter(|(left, right)| left.clone().into_iter().any(|i| right.contains(&i)))
        .count()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {}

    #[test]
    fn test_part2() {}
}
