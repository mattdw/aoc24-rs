use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::{intmap::IntMap, Day};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Object {
    Wall,
    Start,
    End,
    Empty,
}

impl Default for Object {
    fn default() -> Self {
        Object::Empty
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Dir {
    North,
    South,
    East,
    West,
}

fn parse(input: &str) -> (IntMap<Object>, (isize, isize), (isize, isize)) {
    let m = IntMap::from_raw(input, |c| match c {
        '#' => Object::Wall,
        'S' => Object::Start,
        'E' => Object::End,
        '.' => Object::Empty,
        _ => panic!("bad char"),
    });

    let mut start = (-1, -1);
    let mut end = (-1, -1);
    for (idx, n) in m.cells.iter().enumerate() {
        if n == &Object::Start {
            start = m.idx_to_pt(idx as isize);
        }
        if n == &Object::End {
            end = m.idx_to_pt(idx as isize);
        }
    }

    (m, start, end)
}

fn adjacent_dirs(d: Dir) -> [Dir; 2] {
    match d {
        Dir::East | Dir::West => [Dir::North, Dir::South],
        Dir::North | Dir::South => [Dir::East, Dir::West],
    }
}

fn delta(d: Dir) -> (isize, isize) {
    match d {
        Dir::East => (1, 0),
        Dir::West => (-1, 0),
        Dir::North => (0, -1),
        Dir::South => (0, 1),
    }
}

#[derive(Clone, Eq, PartialEq)]
struct QueueState((isize, isize), Dir, i64, HashSet<(u8, u8)>);

impl Ord for QueueState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.2.cmp(&self.2)
    }
}

impl PartialOrd for QueueState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(
    m: &IntMap<Object>,
    start: (isize, isize),
    start_dir: Dir,
    end: (isize, isize),
    first_only: bool,
) -> Option<(i64, HashSet<(u8, u8)>)> {
    let mut seen = HashMap::<((isize, isize), Dir), i64>::new();
    let mut q = BinaryHeap::<QueueState>::new();
    q.push(QueueState(start, start_dir, 0, HashSet::new()));

    let mut best_cost = i64::MAX;
    let mut bests = HashMap::<i64, HashSet<(u8, u8)>>::new();

    loop {
        let Some(QueueState(pos, dir, cost, route)) = q.pop() else {
            break;
        };

        if let Some(&val) = seen.get(&(pos, dir)) {
            if cost > val || first_only {
                continue;
            }
        }

        // record it as seen, with updated lowest cost as necessary
        seen.entry((pos, dir))
            .and_modify(|v| *v = (*v).min(cost))
            .or_insert(cost);

        if cost > best_cost {
            break;
        }

        let mut new_route = route;
        new_route.insert((pos.0 as u8, pos.1 as u8));

        if pos == end {
            // println!("found a route {} / {}", cost, new_route.len());
            best_cost = best_cost.min(cost);
            let r = bests.entry(cost).or_default();
            r.extend(new_route);

            if first_only {
                break;
            } else {
                continue;
            }
        }

        for nextd in adjacent_dirs(dir) {
            q.push(QueueState(pos, nextd, cost + 1000, new_route.clone()));
        }

        // turning is expensive but it's all we can do if we're at the last
        // position, and there's no point wasting time generating forward
        // steps that'll get seen first.
        if pos == end {
            continue;
        }

        let dxy = delta(dir);
        let fwd = (pos.0 + dxy.0, pos.1 + dxy.1);

        if let Some(v) = m.get(fwd) {
            if v != &Object::Wall {
                q.push(QueueState(fwd, dir, cost + 1, new_route));
            }
        }
    }

    // dbg!(&bests);
    let best_set = bests.remove(&best_cost).unwrap();

    Some((best_cost, best_set))
}

pub struct Day16 {}

impl Day<i64> for Day16 {
    fn part1(input: &str) -> i64 {
        let (m, s, e) = parse(input);

        let (cost, _visited) = dijkstra(&m, s, Dir::East, e, true).unwrap();

        cost
    }

    fn part2(input: &str) -> i64 {
        let (m, s, e) = parse(input);

        let (_, visited) = dijkstra(&m, s, Dir::East, e, false).unwrap();

        visited.len() as i64
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &'static str = "
    ###############
    #.......#....E#
    #.#.###.#.###.#
    #.....#.#...#.#
    #.###.#####.#.#
    #.#.#.......#.#
    #.#.#####.###.#
    #...........#.#
    ###.#.#####.#.#
    #...#.....#.#.#
    #.#.#.###.#.#.#
    #.....#...#.#.#
    #.###.#.#.#.#.#
    #S..#.....#...#
    ###############
    ";

    const TEST_INPUT_2: &'static str = "
    #################
    #...#...#...#..E#
    #.#.#.#.#.#.#.#.#
    #.#.#.#...#...#.#
    #.#.#.#.###.#.#.#
    #...#.#.#.....#.#
    #.#.#.#.#.#####.#
    #.#...#.#.#.....#
    #.#.#####.#.###.#
    #.#.#.......#...#
    #.#.###.#####.###
    #.#.#...#.....#.#
    #.#.#.#####.###.#
    #.#.#.........#.#
    #.#.#.#########.#
    #S#.............#
    #################
    ";

    #[test]
    fn part1_small() {
        assert_eq!(7036, Day16::part1(TEST_INPUT));
    }

    #[test]
    fn part2_small() {
        assert_eq!(45, Day16::part2(TEST_INPUT));
    }

    #[test]
    fn part2_medium() {
        assert_eq!(64, Day16::part2(TEST_INPUT_2));
    }

    #[test]
    #[ignore]
    fn bench_clone_hashset() {
        let mut h = HashSet::new();
        h.insert(((1, 2), Dir::East));
        h.insert(((2, 3), Dir::West));
        h.insert(((2, 3), Dir::West));

        for _i in 0..10_000_000 {
            let h2 = h.clone();
            // use both
            assert_eq!(h2.len(), h.len());
            // drop orig
            h = h2;
        }
    }

    #[test]
    #[ignore]
    fn bench_clone_vec() {
        let mut h = Vec::new();
        h.push(((1, 2), Dir::East));
        h.push(((2, 3), Dir::West));
        h.push(((2, 3), Dir::West));

        for _i in 0..10_000_000 {
            let h2 = h.clone();
            assert_eq!(h2.len(), h.len());
            h = h2;
        }
    }
}
