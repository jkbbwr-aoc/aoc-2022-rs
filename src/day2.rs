use std::collections::HashMap;
use aoc_runner_derive::{aoc_generator, aoc};


pub enum Move {
    ROCK,
    PAPER,
    SCISSORS,
}

impl From<char> for Move {
    fn from(c: char) -> Self {
        match c {
            'A' | 'X' => Self::ROCK,
            'B' | 'Y' => Self::PAPER,
            'C' | 'Z' => Self::SCISSORS,
            _ => panic!("unreachable")
        }
    }
}

impl Move {
    fn winner(&self, goal: char) -> Self {
        match (self, goal) {
            (Move::ROCK, 'X') => Move::SCISSORS,
            (Move::ROCK, 'Y') => Move::ROCK,
            (Move::ROCK, 'Z') => Move::PAPER,
            (Move::SCISSORS, 'X') => Move::PAPER,
            (Move::SCISSORS, 'Y') => Move::SCISSORS,
            (Move::SCISSORS, 'Z') => Move::ROCK,
            (Move::PAPER, 'X') => Move::ROCK,
            (Move::PAPER, 'Y') => Move::PAPER,
            (Move::PAPER, 'Z') => Move::SCISSORS,
            _ => panic!("unreachable")
        }
    }

    fn score(&self, other: &Move) -> i32 {
        let total = match (self, other) {
            (Move::ROCK, Move::SCISSORS) => 6,
            (Move::ROCK, Move::ROCK) => 3,
            (Move::ROCK, Move::PAPER) => 0,
            (Move::SCISSORS, Move::PAPER) => 6,
            (Move::SCISSORS, Move::SCISSORS) => 3,
            (Move::SCISSORS, Move::ROCK) => 0,
            (Move::PAPER, Move::ROCK) => 6,
            (Move::PAPER, Move::PAPER) => 3,
            (Move::PAPER, Move::SCISSORS) => 0,
        };
        let shape = match (self) {
            Move::ROCK => 1,
            Move::PAPER => 2,
            Move::SCISSORS => 3
        };
        total + shape
    }
}

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<(char, char)> {
    input.lines().map(|line| {
        let mut chars = line.as_bytes();
        (chars[0 as usize] as char, chars[2 as usize] as char)
    }).collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[(char, char)]) -> i32 {
    input.iter().map(|(left, right)| {
        let them = Move::from(*left);
        let us = Move::from(*right);
        us.score(&them)
    }).sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &[(char, char)]) -> i32 {
    input.iter().map(|(left, right)| {
        let them = Move::from(*left);
        let us = them.winner(*right);
        us.score(&them)
    }).sum()
}