use aoc_runner_derive::{aoc, aoc_generator};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::cmp::Ordering;
use std::iter;

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
#[serde(untagged)]
pub enum Packet {
    Value(i32),
    Nested(Vec<Packet>),
    EOF,
}

impl Packet {
    fn unwrap(&self) -> Option<Self> {
        return match self {
            a @ Packet::Value(_) => Some(a.clone()),
            Packet::Nested(a) => {
                if a.len() != 1 {
                    return None;
                }
                return a.first().cloned();
            }
            Packet::EOF => None,
        };
    }
}

impl PartialOrd<Self> for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match compare_packets(self, other) {
            Validity::Continue => panic!("Fuck"),
            Validity::Valid => Ordering::Less,
            Validity::Invalid => Ordering::Greater,
        }
    }
}

#[derive(Debug)]
enum Validity {
    Continue,
    Valid,
    Invalid,
}

fn compare_packets(left: &Packet, right: &Packet) -> Validity {
    //println!("{:?}\t{:?}", left, right);
    match (left, right) {
        (l @ Packet::Nested(_), r @ Packet::Value(_)) => {
            compare_packets(l, &Packet::Nested(vec![r.clone()]))
        }
        (l @ Packet::Value(_), r @ Packet::Nested(_)) => {
            compare_packets(&Packet::Nested(vec![l.clone()]), r)
        }
        (Packet::Value(l), Packet::Value(r)) if l == r => Validity::Continue,
        (Packet::Value(l), Packet::Value(r)) if l < r => Validity::Valid,
        (Packet::Value(l), Packet::Value(r)) if l > r => Validity::Invalid,
        (Packet::EOF, Packet::EOF) => Validity::Continue,
        (Packet::EOF, _) => Validity::Valid,
        (_, Packet::EOF) => Validity::Invalid,
        (Packet::Nested(l), Packet::Nested(r)) => {
            for (l, r) in l
                .iter()
                .chain(iter::repeat(&Packet::EOF))
                .zip(r.iter().chain(iter::repeat(&Packet::EOF)))
            {
                if let (Packet::EOF, Packet::EOF) = (l, r) {
                    // We have run out of both iterators together. We need to continue
                    return Validity::Continue;
                }

                let state = compare_packets(l, r);
                match state {
                    Validity::Continue => continue,
                    Validity::Valid | Validity::Invalid => return state,
                }
            }
            Validity::Valid
        }
        _ => panic!("oh fuck"),
    }
}

#[aoc_generator(day13)]
pub fn generator(input: &str) -> Vec<(Packet, Packet)> {
    input
        .split("\n\n")
        .map(|chunk| {
            let (first, second) = chunk.trim().split_once('\n').unwrap();
            let a = from_str::<Packet>(first).unwrap();
            let b = from_str::<Packet>(second).unwrap();
            (a, b)
        })
        .collect()
}

#[aoc(day13, part1)]
pub fn part1(input: &[(Packet, Packet)]) -> i32 {
    input
        .iter()
        .enumerate()
        .filter_map(|(index, (left, right))| {
            let index = index + 1;

            match compare_packets(left, right) {
                Validity::Valid => Some(index as i32),
                Validity::Invalid => None,
                _ => panic!("Internal state leak lmao"),
            }
        })
        .sum()
}

#[aoc(day13, part2)]
pub fn part2(input: &[(Packet, Packet)]) -> i32 {
    let a = from_str::<Packet>("[[2]]").unwrap();
    let b = from_str::<Packet>("[[6]]").unwrap();
    let mut packets = vec![a, b];
    input.iter().for_each(|(left, right)| {
        packets.push(left.clone());
        packets.push(right.clone());
    });
    packets.sort();

    packets
        .iter()
        .enumerate()
        .filter_map(|(index, packet)| {
            if let Some(Packet::Value(2)) = packet.unwrap().and_then(|i| i.unwrap()) {
                println!("{:?} {:?}", index, packet);
                Some((index + 1) as i32)
            } else if let Some(Packet::Value(6)) = packet.unwrap().and_then(|i| i.unwrap()) {
                println!("{:?} {:?}", index, packet);
                Some((index + 1) as i32)
            } else {
                None
            }
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day13_ord() {
        let input = r"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";
        let gen = generator(input);
        println!("{}", part2(&gen));
    }

    #[test]
    fn test_day13_compare() {
        let a = from_str::<Packet>(r"[]").unwrap();
        let b = from_str::<Packet>(r"[]").unwrap();
        dbg!(compare_packets(&a, &b));
    }

    #[test]
    fn test_day13() {
        let input = r"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";
        let gen = generator(input);
        for (index, pair) in gen.iter().enumerate() {
            println!("{:?} = {:?}", index + 1, compare_packets(&pair.0, &pair.1))
        }
    }
}
