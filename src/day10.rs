use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
enum Pixel {
    Lit,
    Dark,
}

impl ToString for Pixel {
    fn to_string(&self) -> String {
        match self {
            Pixel::Lit => "█".into(),
            Pixel::Dark => " ".into()
        }
    }
}

#[derive(Debug)]
struct VM<TrapFn>
    where
        TrapFn: FnMut(i32, &mut State) -> (),
{
    cycle: i32,
    trap: TrapFn,
    state: State,
}

#[derive(Debug)]
struct State {
    x: i32,
    row: Vec<Pixel>,
    sprite: i32,
}

impl<TrapFn> VM<TrapFn>
    where
        TrapFn: FnMut(i32, &mut State) -> (),
{
    fn new(trap: TrapFn) -> Self {
        VM {
            cycle: 0,
            trap,
            state: State {
                x: 1,
                row: Vec::new(),
                sprite: 1,
            },
        }
    }

    fn cycle(&mut self) {
        self.cycle += 1;
        (self.trap)(self.cycle, &mut self.state);
    }

    fn add(&mut self, i: i32) {
        self.cycle();
        self.cycle();
        self.state.x += i;
        self.state.sprite = self.state.x;
    }

    fn noop(&mut self) {
        self.cycle();
    }

    fn run(mut self, program: &[Instruction]) {
        for instruction in program {
            match instruction {
                Instruction::Addx(i) => self.add(*i),
                Instruction::Noop => self.noop(),
            };
        }
    }
}

#[derive(Debug)]
pub enum Instruction {
    Addx(i32),
    Noop,
}

#[aoc_generator(day10)]
pub fn generator(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| match line.trim().split_once(" ") {
            Some(("addx", i)) => Instruction::Addx(i.parse().unwrap()),
            None => Instruction::Noop,
            _ => panic!("Invalid instruction."),
        })
        .collect()
}

#[aoc(day10, part1)]
pub fn part1(program: &[Instruction]) -> i32 {
    let mut strength = 0;
    let cycles = [20, 60, 100, 140, 180, 220];

    let vm = VM::new(
        |cycle, state| if cycles.contains(&cycle) {
            strength += cycle * state.x;
        },
    );
    vm.run(program);
    strength
}

#[aoc(day10, part2)]
pub fn part2(program: &[Instruction]) -> String {
    let mut screen = Vec::from([String::new(); 1]);
    let vm = VM::new(
        |cycle, state| {
            if (state.sprite - 1..=state.sprite + 1).contains(&(state.row.len() as i32)) {
                state.row.push(Pixel::Lit);
            } else {
                state.row.push(Pixel::Dark);
            }
            if cycle % 40 == 0 {
                let row = state.row.iter().map(|p| p.to_string()).collect::<Vec<_>>().join("");
                screen.push(row);
                state.row.clear();
            }
        });
    vm.run(program);
    screen.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day10() {
        let input = r"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
        println!("{:?}", part1(&generator(&input)));
    }
}
