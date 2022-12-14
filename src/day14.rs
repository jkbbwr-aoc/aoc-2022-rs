use aoc_runner_derive::aoc;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Space {
    Rock,
    Air,
    Sand,
    Source,
}

#[allow(dead_code)]
fn draw_grid(grid: &HashMap<(i32, i32), Space>) {
    let min_x = grid.keys().min_by_key(|(x, _y)| x).unwrap().0;
    let min_y = grid.keys().min_by_key(|(_x, y)| y).unwrap().1;
    let max_x = grid.keys().max_by_key(|(x, _y)| x).unwrap().0;
    let max_y = grid.keys().max_by_key(|(_x, y)| y).unwrap().1;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let space = grid.get(&(x, y)).unwrap_or(&Space::Air);
            match space {
                Space::Rock => print!("█"),
                Space::Air => print!("░"),
                Space::Sand => print!("o"),
                Space::Source => print!("+"),
            }
        }
        println!();
    }
}

pub fn generator(input: &str) -> HashMap<(i32, i32), Space> {
    let mut map = HashMap::new();
    map.insert((500, 0), Space::Source);
    let build = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|pos| {
                    let (x, y) = pos.split_once(',').unwrap();
                    (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for wall in build {
        for pair in wall.windows(2) {
            let (x1, y1) = pair[0];
            let (x2, y2) = pair[1];
            for x in min(x1, x2)..=max(x1, x2) {
                for y in min(y1, y2)..=max(y1, y2) {
                    map.insert((x, y), Space::Rock);
                }
            }
        }
    }

    map
}

fn fall(pos: &mut (i32, i32), grid: &mut HashMap<(i32, i32), Space>) -> bool {
    // given a position. drop it to the bottom. do not do diagonal checks.
    loop {
        let max_y = grid.keys().max_by_key(|(_x, y)| y).unwrap().1;
        if pos.1 > max_y {
            return true;
        }
        let space = grid.get(&(pos.0, pos.1 + 1)).unwrap_or(&Space::Air);
        match space {
            Space::Rock | Space::Sand | Space::Source => return false,
            Space::Air => pos.1 += 1,
        }
    }
}

fn simulate_single_sand(
    pos: &mut (i32, i32),
    grid: &mut HashMap<(i32, i32), Space>,
    floor: i32,
) -> bool {
    if floor != 0 {
        grid.insert((pos.0 - 1, floor + 2), Space::Rock);
        grid.insert((pos.0, floor + 2), Space::Rock);
        grid.insert((pos.0 + 1, floor + 2), Space::Rock);
    }
    // first go til we hit rock or sand.
    let infinite = fall(pos, grid);
    if infinite {
        return true;
    }
    // can we go left. keep going until we can't anymore
    let space = grid.get(&(pos.0 - 1, pos.1 + 1)).unwrap_or(&Space::Air);
    if let Space::Air = space {
        pos.0 -= 1;
        pos.1 += 1;
        let infinite = fall(pos, grid);
        if infinite {
            return true;
        }
        return simulate_single_sand(pos, grid, floor);
    }

    // can we go right.
    let space = grid.get(&(pos.0 + 1, pos.1 + 1)).unwrap_or(&Space::Air);

    if let Space::Air = space {
        pos.0 += 1;
        pos.1 += 1;
        let infinite = fall(pos, grid);
        if infinite {
            return true;
        }
        return simulate_single_sand(pos, grid, floor);
    }

    if (500, 0) == *pos {
        return true;
    }
    grid.insert((pos.0, pos.1), Space::Sand);

    false
}

#[aoc(day14, part1)]
pub fn part1(input: &str) -> i32 {
    let mut grid = generator(input);

    let mut i = 0;
    loop {
        if simulate_single_sand(&mut (500, 1), &mut grid, 0) {
            return i;
        }
        i += 1;
    }
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> i32 {
    let mut grid = generator(input);
    let max_y = grid.keys().max_by_key(|(_x, y)| y).unwrap().1;

    let mut i = 1;
    loop {
        if simulate_single_sand(&mut (500, 0), &mut grid, max_y) {
            let max_x = grid.keys().max_by_key(|(x, _y)| x).unwrap().0;
            let min_x = grid.keys().min_by_key(|(x, _y)| x).unwrap().0;
            println!("x ={max_x} x = {min_x}");
            return i;
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day14() {
        let input = r"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
        println!("{}", part2(input));
    }
}
