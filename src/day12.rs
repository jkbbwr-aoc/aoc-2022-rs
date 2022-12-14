use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::matrix::Matrix;
use pathfinding::prelude::dijkstra;

#[aoc_generator(day12)]
pub fn generator(input: &str) -> Matrix<char> {
    Matrix::from_rows(input.lines().map(|l| l.chars())).unwrap()
}

fn do_the_thing(matrix: &Matrix<char>, start: (usize, usize), end: (usize, usize)) -> i32 {
    let (_path, n) = dijkstra(
        &start,
        |&pos| {
            let myself = *matrix.get(pos).unwrap();
            let neighbours = matrix.neighbours(pos, false);
            neighbours
                .filter_map(|pos| {
                    let them = *matrix.get(pos).unwrap();
                    match (myself, them) {
                        _ if myself == them => Some((pos, 1)),
                        _ if myself as i32 + 1 == them as i32 => Some((pos, 1)),
                        ('S', 'a') => Some((pos, 1)),
                        ('z', 'E') => Some((pos, 1)),
                        (_myself, 'E') => None,
                        (myself, them) if myself as i32 > them as i32 => {
                            Some((pos, (myself as i32 - them as i32)))
                        }
                        _ => None,
                    }
                })
                .collect::<Vec<_>>()
        },
        |&p| p == end,
    )
    .unwrap();
    n - 1
}

fn do_the_thing_but_shorter(matrix: &Matrix<char>, start: (usize, usize)) -> i32 {
    dijkstra(
        &start,
        |&pos| {
            let myself = *matrix.get(pos).unwrap();
            let neighbours = matrix.neighbours(pos, false);
            neighbours.filter_map(move |pos| {
                let them = *matrix.get(pos).unwrap();
                match (myself, them) {
                    _ if myself == them => Some((pos, 1)),
                    _ if them as i32 + 1 == myself as i32 => Some((pos, 1)),
                    ('a', 'S') => Some((pos, 1)),
                    ('E', 'z') => Some((pos, 1)),
                    (_myself, 'E') => None,
                    (myself, them) if (myself as i32) < (them as i32) => {
                        Some((pos, (them as i32 - myself as i32)))
                    }
                    _ => None,
                }
            })
        },
        |&p| *matrix.get(p).unwrap() == 'a',
    )
    .unwrap()
    .1 - 1
}

#[aoc(day12, part1)]
pub fn part1(matrix: &Matrix<char>) -> i32 {
    let start = matrix
        .keys()
        .find(|&i| *matrix.get(i).unwrap() == 'S')
        .unwrap();
    let end = matrix
        .keys()
        .find(|&i| *matrix.get(i).unwrap() == 'E')
        .unwrap();

    do_the_thing(matrix, start, end)
}

#[aoc(day12, part2)]
pub fn part2(matrix: &Matrix<char>) -> i32 {
    let mut matrix = matrix.clone();
    let start = matrix
        .keys()
        .find(|&i| *matrix.get(i).unwrap() == 'S')
        .unwrap();

    *matrix.get_mut(start).unwrap() = 'a';

    let start = matrix
        .keys()
        .find(|&i| *matrix.get(i).unwrap() == 'E')
        .unwrap();

    do_the_thing_but_shorter(&matrix, start)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day12() {
        let input = r"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        let grid = generator(input);
        println!("{:?}", part2(&grid));
    }
}
