use std::collections::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone)]
pub struct Instruction {
    amount: i32,
    from: i32,
    to: i32,
}

#[derive(Debug, Clone)]
pub struct Stacks(HashMap<i32, Vec<char>>);

impl Deref for Stacks {
    type Target = HashMap<i32, Vec<char>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Stacks {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone)]
pub struct Environment {
    stacks: Stacks,
    instructions: Vec<Instruction>,
}

impl Environment {
    pub fn run_simple(&mut self) {
        for instruction in &self.instructions {
            for _ in 1..=instruction.amount {
                let value = self.stacks.get_mut(&instruction.from).unwrap().pop().unwrap();
                self.stacks.get_mut(&instruction.to).unwrap().push(value);
            }
        }
    }

    pub fn run_advanced(&mut self) {
        for instruction in &self.instructions {
            let outgoing = self.stacks.get_mut(&instruction.from).unwrap();
            let mut crates = outgoing.drain(outgoing.len() - instruction.amount as usize..).collect::<Vec<char>>();
            self.stacks.get_mut(&instruction.to).unwrap().append(&mut crates);
        }
    }
}

#[aoc_generator(day5)]
pub fn generator(input: &str) -> Environment {
    let mut stacks = Stacks(HashMap::new());
    let sections = input.split("\n\n").collect::<Vec<_>>();
    let layout = sections.get(0).unwrap();
    let raw_instructions = sections.get(1).unwrap();

    let mut lines = layout.lines().peekable();
    while let Some(line) = lines.next() {
        if lines.peek().is_some() {
            for (index, item) in line.chars().skip(1).step_by(4).enumerate() {
                if item == ' ' {
                    continue;
                }
                let stack = stacks.entry((index as i32) + 1).or_insert(Vec::new());
                stack.push(item);
            }
        }
    }

    // reverse all the stacks
    for stack in stacks.values_mut() {
        stack.reverse()
    }

    let instructions = raw_instructions.lines().map(|line| {
        let inst  = line.split(" ").filter_map(|i| i.parse::<i32>().ok()).collect::<Vec<_>>();
        Instruction {
            amount: *inst.get(0).unwrap(),
            from: *inst.get(1).unwrap(),
            to: *inst.get(2).unwrap(),
        }
    }).collect::<Vec<_>>();

    return Environment {
        stacks,
        instructions,
    };
}

#[aoc(day5, part1)]
pub fn part1(input: &Environment) -> String {
    let mut env = input.clone(); // Workaround for non mutable input.
    env.run_simple();
    let end = env.stacks.keys().max().unwrap();
    (1..=*end).map(|i| {
        env.stacks.get(&i).unwrap().last().unwrap()
    }).collect::<String>()
}

#[aoc(day5, part2)]
pub fn part2(input: &Environment) -> String {
    let mut env = input.clone(); // Workaround for non mutable input.
    env.run_advanced();
    let end = env.stacks.keys().max().unwrap();
    (1..=*end).map(|i| {
        env.stacks.get(&i).unwrap().last().unwrap()
    }).collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator() {
        let a = r"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        println!("{:?}", part1(&generator(a)));
    }
}
