use aoc_runner_derive::{aoc, aoc_generator};

use std::collections::HashSet;

#[derive(Debug, Clone)]
pub enum Instruction {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

#[derive(Debug, Clone)]
pub struct World {
    head: Position,
    tail: Position,
    visited: HashSet<Position>,
    instructions: Vec<Instruction>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Position(i32, i32);

fn follow(head: Position, tail: &mut Position) {
    let difference = (head.0 - tail.0, head.1 - tail.1);
    if difference.0.abs() == 2 || difference.1.abs() == 2 {
        tail.0 += difference.0.signum();
        tail.1 += difference.1.signum();
    }
}

impl World {
    fn simulate_knots(&mut self) {
        let mut knots = vec![Position(0, 0); 10];
        while let Some(instruction) = self.instructions.pop() {
            match instruction {
                Instruction::Up(i) => {
                    for _ in 1..=i {
                        knots[0].0 += 1;
                        for i in 1..knots.len() {
                            follow(knots[i - 1], &mut knots[i]);
                        }
                        self.visited.insert(*knots.last().unwrap());
                    }
                }
                Instruction::Down(i) => {
                    for _ in 1..=i {
                        knots[0].0 -= 1;

                        for i in 1..knots.len() {
                            follow(knots[i - 1], &mut knots[i]);
                        }
                        self.visited.insert(*knots.last().unwrap());
                    }
                }
                Instruction::Left(i) => {
                    for _ in 1..=i {
                        knots[0].1 -= 1;

                        for i in 1..knots.len() {
                            follow(knots[i - 1], &mut knots[i]);
                        }
                        self.visited.insert(*knots.last().unwrap());
                    }
                }
                Instruction::Right(i) => {
                    for _ in 1..=i {
                        knots[0].1 += 1;

                        for i in 1..knots.len() {
                            follow(knots[i - 1], &mut knots[i]);
                        }
                        self.visited.insert(*knots.last().unwrap());
                    }
                }
            }
        }
    }

    fn simulate_rope(&mut self) {
        while let Some(instruction) = self.instructions.pop() {
            match instruction {
                Instruction::Up(i) => {
                    for _ in 1..=i {
                        self.head.0 += 1;
                        follow(self.head, &mut self.tail);
                        self.visited.insert(self.tail);
                    }
                }
                Instruction::Down(i) => {
                    for _ in 1..=i {
                        self.head.0 -= 1;
                        follow(self.head, &mut self.tail);
                        self.visited.insert(self.tail);
                    }
                }
                Instruction::Left(i) => {
                    for _ in 1..=i {
                        self.head.1 -= 1;
                        follow(self.head, &mut self.tail);
                        self.visited.insert(self.tail);
                    }
                }
                Instruction::Right(i) => {
                    for _ in 1..=i {
                        self.head.1 += 1;
                        follow(self.head, &mut self.tail);
                        self.visited.insert(self.tail);
                    }
                }
            }
        }
    }
}

#[aoc_generator(day9)]
pub fn generator(input: &str) -> World {
    let visited = HashSet::new();
    let head = Position(0, 0);
    let tail = Position(0, 0);
    let instructions = input
        .lines()
        .map(|line| match line.trim().split_once(' ').unwrap() {
            ("L", i) => Instruction::Left(i.parse().unwrap()),
            ("R", i) => Instruction::Right(i.parse().unwrap()),
            ("U", i) => Instruction::Up(i.parse().unwrap()),
            ("D", i) => Instruction::Down(i.parse().unwrap()),
            _ => panic!("Fucked instruction."),
        })
        .rev() // fuck you reverse the list so I can pop.
        .collect();

    World {
        head,
        tail,
        visited,
        instructions,
    }
}

#[aoc(day9, part1)]
pub fn part1(world: &World) -> i32 {
    let mut world = world.clone();
    world.simulate_rope();
    world.visited.len() as i32
}

#[aoc(day9, part2)]
pub fn part2(world: &World) -> i32 {
    let mut world = world.clone();
    world.simulate_knots();
    world.visited.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day9() {
        let input = r"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        let world = generator(input);
        println!("{:?}", part2(&world));
    }
}
