use aoc_runner_derive::aoc;
use std::collections::VecDeque;
use std::ops::Fn;

pub struct Monkey {
    items: VecDeque<i128>,
    op: Box<dyn Fn(i128) -> i128>,
    test: Box<dyn Fn(i128) -> i128>,
    worry_factor: i128,
    inspections: i128,
}

fn round(index: usize, monkeys: &mut [Monkey]) {
    let (start, end) = monkeys.split_at_mut(index);
    let (monkey, end) = end.split_first_mut().unwrap();
    while let Some(item) = monkey.items.pop_front() {
        monkey.inspections += 1;
        let worry = (monkey.op)(item) as i128;
        let worry = worry / monkey.worry_factor;
        let worry = if monkey.worry_factor == 1 {
            worry % 9699690 // what the fuck?
        } else {
            worry
        };
        let target = (monkey.test)(worry) as usize;
        let target_monkey = if target < index {
            &mut start[target]
        } else {
            &mut end[target - index - 1]
        };
        target_monkey.items.push_back(worry);
    }
}

pub fn generator(input: &str, worry_factor: i128) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|chunk| {
            let mut lines = chunk.lines();
            let _id_line = lines.next().unwrap();
            let mut items = lines
                .next()
                .unwrap()
                .split_once(": ")
                .map(|(_, items)| {
                    items
                        .split(",")
                        .map(|i| i.trim().parse::<i128>().unwrap())
                        .collect::<VecDeque<_>>()
                })
                .unwrap();
            let operation_line = lines
                .next()
                .unwrap()
                .trim()
                .split(" ")
                .skip(1)
                .collect::<Vec<_>>();
            let test_line = lines
                .next()
                .unwrap()
                .trim()
                .split(" ")
                .skip(1)
                .collect::<Vec<_>>();
            let if_true_line = lines
                .next()
                .unwrap()
                .trim()
                .split(" ")
                .skip(1)
                .collect::<Vec<_>>();
            let if_false_line = lines
                .next()
                .unwrap()
                .trim()
                .split(" ")
                .skip(1)
                .collect::<Vec<_>>();

            let test: Box<dyn Fn(i128) -> i128> = if let ["divisible", "by", div] = test_line[..] {
                if let ["true:", "throw", "to", "monkey", true_id] = if_true_line[..] {
                    if let ["false:", "throw", "to", "monkey", false_id] = if_false_line[..] {
                        let div = div.parse::<i128>().unwrap();
                        let a = true_id.parse::<i128>().unwrap();
                        let b = false_id.parse::<i128>().unwrap();
                        Box::new(move |i| if i % div == 0 { a } else { b })
                    } else {
                        panic!("Failed to match false");
                    }
                } else {
                    panic!("Failed to match true");
                }
            } else {
                panic!("Failed to match div");
            };

            let op: Box<dyn Fn(i128) -> i128> = match operation_line[..] {
                ["new", "=", "old", "+", num] => {
                    if num != "old" {
                        let num = num.parse::<i128>().unwrap();
                        Box::new(move |i| i + num)
                    } else {
                        Box::new(move |i| i + i)
                    }
                }
                ["new", "=", "old", "-", num] => {
                    if num != "old" {
                        let num = num.parse::<i128>().unwrap();
                        Box::new(move |i| i - num)
                    } else {
                        Box::new(move |i| i - i)
                    }
                }
                ["new", "=", "old", "*", num] => {
                    if num != "old" {
                        let num = num.parse::<i128>().unwrap();
                        Box::new(move |i| {
                            i * num
                        })
                    } else {
                        Box::new(move |i| {
                            i * i
                        })
                    }
                }
                ["new", "=", "old", "/", num] => {
                    if num != "old" {
                        let num = num.parse::<i128>().unwrap();
                        Box::new(move |i| i / num)
                    } else {
                        Box::new(move |i| i / i)
                    }
                }
                _ => panic!("wtf"),
            };

            Monkey {
                items,
                op,
                test,
                worry_factor,
                inspections: 0,
            }
        })
        .collect()
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> i128 {
    let mut monkeys = generator(input, 3);
    let size = monkeys.len();
    for _ in 0..20 {
        for index in 0..size {
            round(index, &mut monkeys);
        }
    }
    monkeys.sort_by_key(|m| std::cmp::Reverse(m.inspections));
    monkeys[0..2].iter().map(|m| m.inspections).product()
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> i128 {
    let mut monkeys = generator(input, 1);
    let size = monkeys.len();
    for _ in 0..10000 {
        for index in 0..size {
            round(index, &mut monkeys);
        }
    }
    monkeys.sort_by_key(|m| std::cmp::Reverse(m.inspections));
    monkeys[0..2].iter().map(|m| m.inspections).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day11() {
        let input = r"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
        println!("{}", part2(&input));
    }
}
