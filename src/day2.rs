use aoc_runner_derive::{aoc, aoc_generator};

pub enum Move {
    Rock,
    Paper,
    Scissors,
}

impl From<char> for Move {
    fn from(c: char) -> Self {
        match c {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => panic!("unreachable"),
        }
    }
}

impl Move {
    fn winner(&self, goal: char) -> Self {
        match (self, goal) {
            (Move::Rock, 'X') => Move::Scissors,
            (Move::Rock, 'Y') => Move::Rock,
            (Move::Rock, 'Z') => Move::Paper,
            (Move::Scissors, 'X') => Move::Paper,
            (Move::Scissors, 'Y') => Move::Scissors,
            (Move::Scissors, 'Z') => Move::Rock,
            (Move::Paper, 'X') => Move::Rock,
            (Move::Paper, 'Y') => Move::Paper,
            (Move::Paper, 'Z') => Move::Scissors,
            _ => panic!("unreachable"),
        }
    }

    fn score(&self, other: &Move) -> i32 {
        let total = match (self, other) {
            (Move::Rock, Move::Scissors) => 6,
            (Move::Rock, Move::Rock) => 3,
            (Move::Rock, Move::Paper) => 0,
            (Move::Scissors, Move::Paper) => 6,
            (Move::Scissors, Move::Scissors) => 3,
            (Move::Scissors, Move::Rock) => 0,
            (Move::Paper, Move::Rock) => 6,
            (Move::Paper, Move::Paper) => 3,
            (Move::Paper, Move::Scissors) => 0,
        };
        let shape = match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        };
        total + shape
    }
}

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<(char, char)> {
    input
        .lines()
        .map(|line| {
            let chars = line.as_bytes();
            (chars[0_usize] as char, chars[2_usize] as char)
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[(char, char)]) -> i32 {
    input
        .iter()
        .map(|(left, right)| {
            let them = Move::from(*left);
            let us = Move::from(*right);
            us.score(&them)
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &[(char, char)]) -> i32 {
    input
        .iter()
        .map(|(left, right)| {
            let them = Move::from(*left);
            let us = them.winner(*right);
            us.score(&them)
        })
        .sum()
}
