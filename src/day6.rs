use aoc_runner_derive::aoc;
use std::collections::HashSet;

fn solve_for_size<const SIZE: usize>(input: &str) -> i32 {
    let chars = input.chars().collect::<Vec<_>>();
    let slice = &chars;
    let mut find = slice
        .windows(SIZE)
        .enumerate()
        .filter_map(|(index, window)| {
            let set: HashSet<char> =
                HashSet::from(<&[char] as TryInto<[char; SIZE]>>::try_into(window).unwrap());
            if set.len() == SIZE {
                Some((index, window))
            } else {
                None
            }
        });
    let (index, _) = find.next().unwrap();
    (index + SIZE) as i32
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> i32 {
    solve_for_size::<4>(input)
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> i32 {
    solve_for_size::<14>(input)
}

#[cfg(test)]
mod tests {}
