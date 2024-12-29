use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::{intmap::IntMap, Day};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
enum Object {
    Wall,
    Start,
    End,
    #[default]
    Empty,
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
struct QueueState(i32, (isize, isize), Dir, HashSet<(u8, u8)>);

impl Ord for QueueState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for QueueState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn manhattan(src: &(isize, isize), dest: &(isize, isize)) -> isize {
    (dest.0 - src.0).abs() + (dest.1 - src.1).abs()
}

fn dijkstra(
    m: &IntMap<Object>,
    start: (isize, isize),
    start_dir: Dir,
    end: (isize, isize),
    first_only: bool,
) -> Option<(i64, HashSet<(u8, u8)>)> {
    let mut seen = HashMap::<((isize, isize), Dir), i32>::new();
    let mut q = BinaryHeap::<QueueState>::with_capacity(4096);
    q.push(QueueState(0, start, start_dir, HashSet::new()));

    let mut best_cost = i32::MAX;
    let mut bests = HashMap::<i32, HashSet<(u8, u8)>>::new();

    loop {
        let Some(QueueState(cost, pos, dir, route)) = q.pop() else {
            break;
        };

        if cost > best_cost {
            break;
        }

        // manhattan should be an appropriate minimum cost
        let mut cost_est = manhattan(&pos, &end) as i32;
        if pos.0 != end.0 && pos.1 != end.1 {
            cost_est += 1000;
        }
        if cost_est > best_cost {
            continue;
        }

        if let Some(&val) = seen.get(&(pos, dir)) {
            if cost > val || first_only {
                continue;
            }
        }

        // record it as seen, with updated lowest cost as necessary
        seen.entry((pos, dir))
            .and_modify(|v| *v = (*v).min(cost))
            .or_insert(cost);

        let mut route = route;
        route.insert((pos.0 as u8, pos.1 as u8));

        if pos == end {
            // println!("found a route {} / {}", cost, new_route.len());
            best_cost = best_cost.min(cost);
            let r = bests.entry(cost).or_default();
            r.extend(route);

            if first_only {
                break;
            } else {
                continue;
            }
        }

        for nextd in adjacent_dirs(dir) {
            q.push(QueueState(cost + 1000, pos, nextd, route.clone()));
        }

        let fwd = {
            let dxy = delta(dir);
            (pos.0 + dxy.0, pos.1 + dxy.1)
        };
        if let Some(v) = m.get(fwd) {
            if *v != Object::Wall {
                q.push(QueueState(cost + 1, fwd, dir, route));
            }
        }
    }

    let best_set = bests.remove(&best_cost).unwrap();

    Some((best_cost as i64, best_set))
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

    const TEST_INPUT: &str = "
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

    const TEST_INPUT_2: &str = "
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
