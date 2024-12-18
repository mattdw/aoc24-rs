use std::collections::{BinaryHeap, HashSet};

use crate::Day;

type P = (i32, i32);

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct QueueState(P, i32);

impl Ord for QueueState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.1.cmp(&self.1)
    }
}

impl PartialOrd for QueueState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Day18 {
    bytes: Vec<P>,
    size: P,
    start: P,
    end: P,
}

impl Day18 {
    fn parse(input: &str, size: Option<P>) -> Self {
        let pairs: Vec<(i32, i32)> = input
            .trim()
            .lines()
            .map(|l| {
                let (left, right) = l.trim().split_once(",").unwrap();

                (left.parse().unwrap(), right.parse().unwrap())
            })
            .collect();

        let size = size.unwrap_or((71, 71));
        Day18 {
            bytes: pairs,
            size,
            start: (0, 0),
            end: (size.0 - 1, size.1 - 1),
        }
    }

    fn dijkstra(s: P, e: P, obstacles: &HashSet<P>, bounds: P) -> i32 {
        let mut q = BinaryHeap::<QueueState>::new();
        let mut seen = HashSet::<P>::new();

        q.push(QueueState(s, 0));

        loop {
            let Some(QueueState(p, cost)) = q.pop() else {
                break;
            };

            if p.0 < 0 || p.0 >= bounds.0 || p.1 < 0 || p.1 >= bounds.1 {
                continue;
            }

            if obstacles.contains(&p) {
                continue;
            }

            if p == e {
                return cost;
            }

            if seen.contains(&p) {
                continue;
            }
            seen.insert(p);

            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                q.push(QueueState((p.0 + dx, p.1 + dy), cost + 1));
            }
        }

        -1
    }
}

impl Day<String> for Day18 {
    fn part1(input: &str) -> String {
        let d = Day18::parse(input, None);
        let mut corrupt = HashSet::<P>::new();

        for p in d.bytes.iter().take(1024) {
            corrupt.insert(*p);
        }

        format!("{}", Day18::dijkstra(d.start, d.end, &corrupt, d.size))
    }

    fn part2(input: &str) -> String {
        let d = Day18::parse(input, None);
        let mut corrupt = HashSet::<P>::new();

        // let mut fatal_byte: Option<P> = None;
        // for p in d.bytes.iter() {
        //     corrupt.insert(*p);
        //     let res = Day18::dijkstra(d.start, d.end, &corrupt, d.size);
        //     if res == -1 {
        //         fatal_byte = Some(*p);
        //         break;
        //     }
        // }

        let mut low = 0;
        let mut high = d.bytes.len();
        let fatal_byte;
        loop {
            let mid = (low + high) / 2;
            corrupt.clear();
            for i in 0..mid {
                corrupt.insert(d.bytes[i]);
            }
            let res = Day18::dijkstra(d.start, d.end, &corrupt, d.size);
            if res == -1 {
                // fatal_byte = Some(d.bytes[mid]);
                high = mid;
            } else {
                low = mid + 1;
            }

            if low >= high {
                fatal_byte = Some(d.bytes[mid]);
                break;
            }
        }

        if let Some((x, y)) = fatal_byte {
            format!("{x},{y}")
        } else {
            "-".to_string()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &'static str = "
        5,4
        4,2
        4,5
        3,0
        2,1
        6,3
        2,4
        1,5
        0,6
        3,3
        2,6
        5,1
        1,2
        5,5
        2,5
        6,5
        1,4
        0,4
        6,4
        1,1
        6,1
        1,0
        0,5
        1,6
        2,0
    ";

    const TEST_SIZE: P = (7, 7);

    #[test]
    fn p1_t() {
        let d = Day18::parse(&TEST_INPUT, Some(TEST_SIZE));
        let mut corrupt = HashSet::<P>::new();

        for p in d.bytes.iter().take(12) {
            corrupt.insert(*p);
        }

        assert_eq!(
            22,
            Day18::dijkstra((0, 0), (6, 6), &corrupt, d.size) as isize
        );
    }

    #[test]
    fn p2_t() {
        let d = Day18::parse(&TEST_INPUT, Some(TEST_SIZE));
        let mut corrupt = HashSet::<P>::new();

        // naive solution is linear search

        // for p in d.bytes.iter() {
        //     corrupt.insert(*p);
        //     let res = Day18::dijkstra((0, 0), (6, 6), &corrupt, d.size);
        //     if res == -1 {
        //         fatal_byte = Some(*p);
        //         break;
        //     }
        // }

        // but binary search should be pretty easy?

        let mut low = 0;
        let mut high = d.bytes.len();
        let fatal_byte;
        loop {
            let mid = (low + high) / 2;
            corrupt.clear();
            for i in 0..mid {
                corrupt.insert(d.bytes[i]);
            }
            let res = Day18::dijkstra(d.start, d.end, &corrupt, d.size);
            if res == -1 {
                // fatal_byte = Some(d.bytes[mid]);
                high = mid;
            } else {
                low = mid + 1;
            }

            if low >= high {
                fatal_byte = Some(d.bytes[mid]);
                break;
            }
        }

        assert_eq!(Some((6, 1)), fatal_byte);
    }
}
