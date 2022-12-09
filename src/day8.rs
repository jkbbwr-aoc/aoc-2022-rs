use aoc_runner_derive::{aoc, aoc_generator};
use derive_more::Deref;
use std::collections::HashMap;

#[derive(Debug, Deref)]
pub struct Grid(HashMap<(usize, usize), u32>);

impl Grid {
    fn max(&self) -> (usize, usize) {
        *self.keys().max().unwrap()
    }

    fn can_walk_to_edge<F>(&self, start: (usize, usize), dir: F) -> bool
    where
        F: (Fn((usize, usize)) -> (usize, usize)),
    {
        if let (0, _) | (_, 0) = start {
            return true;
        }
        let mut trees = vec![];
        let mut current = dir(start);
        let size = self[&start];
        while let Some(height) = self.get(&current) {
            trees.push(height);
            if current.0 == 0 || current.1 == 0 {
                break;
            }
            current = dir(current);
        }

        return trees.iter().all(|height| *height < &size);
    }

    fn scenic_til_edge<F>(&self, start: (usize, usize), dir: F) -> i32
    where
        F: (Fn((usize, usize)) -> (usize, usize)),
    {
        if let (0, _) | (_, 0) = start {
            return 0;
        }

        let mut current = dir(start);
        let size = self[&start];
        let mut score = 1;
        while let Some(height) = self.get(&current) {
            if height >= &size {
                return score;
            }
            if current.0 == 0 || current.1 == 0 {
                return score;
            }
            current = dir(current);
            score += 1;
        }
        score - 1
    }

    fn scenic_all_directions(&self, start: (usize, usize)) -> i32 {
        self.scenic_til_edge(start, |(x, y)| (x, y - 1))
            * self.scenic_til_edge(start, |(x, y)| (x + 1, y))
            * self.scenic_til_edge(start, |(x, y)| (x - 1, y))
            * self.scenic_til_edge(start, |(x, y)| (x, y + 1))
    }

    fn can_walk_to_all_edges(&self, start: (usize, usize)) -> bool {
        self.can_walk_to_edge(start, |(x, y)| (x - 1, y))
            || self.can_walk_to_edge(start, |(x, y)| (x + 1, y))
            || self.can_walk_to_edge(start, |(x, y)| (x, y - 1))
            || self.can_walk_to_edge(start, |(x, y)| (x, y + 1))
    }

    fn most_scenic(&self) -> i32 {
        let mut current = 0;
        let (max_x, max_y) = self.max();
        for x in 0..=max_x {
            for y in 0..=max_y {
                let new = self.scenic_all_directions((x, y));
                if new > current {
                    current = new;
                }
            }
        }
        current
    }

    fn count_visible(&self) -> i32 {
        let mut count = 0;
        let (max_x, max_y) = self.max();
        for x in 0..=max_x {
            for y in 0..=max_y {
                if self.can_walk_to_all_edges((x, y)) {
                    count += 1;
                }
            }
        }
        count
    }
}

#[aoc_generator(day8)]
pub fn generator(input: &str) -> Grid {
    let mut out = HashMap::new();
    for (row, line) in input.lines().enumerate() {
        for (col, num) in line.trim().chars().enumerate() {
            out.insert((row, col), num.to_digit(10).unwrap());
        }
    }

    Grid(out)
}

#[aoc(day8, part1)]
pub fn part1(grid: &Grid) -> i32 {
    grid.count_visible()
}

#[aoc(day8, part2)]
pub fn part2(grid: &Grid) -> i32 {
    grid.most_scenic()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day8() {
        let input = r"30373
        25512
        65332
        33549
        35390";
        let grid = generator(input);

        println!("{}", grid.most_scenic());
    }
}
