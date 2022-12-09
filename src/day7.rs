use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug)]
pub struct File {
    path: String,
    size: i32,
}

#[aoc_generator(day7)]
pub fn generator(input: &str) -> Vec<File> {
    let lines = input.lines();
    let mut path = PathBuf::from("/");
    let mut files = Vec::new();
    for line in lines {
        match line.trim() {
            "$ cd .." => {
                path.pop();
            }
            "$ cd /" => {
                while path.parent().is_some() {
                    path.pop();
                }
            }
            cd if cd.starts_with("$ cd") => {
                let dir = cd.split(' ').last().unwrap();
                path.push(dir);
            }
            "$ ls" => {}
            entry => {
                let (probably_size, name) = entry.split_once(' ').unwrap();
                if probably_size != "dir" {
                    path.push(name);
                    files.push(File {
                        path: path.to_str().unwrap().into(),
                        size: probably_size.parse().unwrap(),
                    });
                    path.pop();
                }
            }
        }
    }

    files
}

fn size(files: &[File]) -> HashMap<String, i32> {
    let mut sizes: HashMap<String, i32> = HashMap::new();
    for file in files {
        let mut path = PathBuf::from(&file.path);
        while let Some(cwd) = path.parent() {
            sizes
                .entry(cwd.to_str().unwrap().into())
                .and_modify(|i| {
                    *i += file.size;
                })
                .or_insert(file.size);
            path.pop();
        }
    }
    sizes
}

#[aoc(day7, part1)]
pub fn part1(input: &[File]) -> i32 {
    let sizes = size(input);
    sizes.values().filter(|v| **v <= 100000).sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &[File]) -> i32 {
    let sizes = size(input);
    let have = sizes["/"];
    *sizes
        .values()
        .filter(|v| **v >= (have - 40000000))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day7() {
        let input = r"$ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k";
        println!("{}", part2(&generator(input)));
    }
}
