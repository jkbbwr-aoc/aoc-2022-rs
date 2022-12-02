use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|n| n.trim().parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .max()
        .unwrap()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let mut totals = input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|n| n.trim().parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();
    totals.sort_by(|a, b| b.cmp(a));

    let (winners, _) = totals.split_at(3);
    winners.iter().sum()
}
