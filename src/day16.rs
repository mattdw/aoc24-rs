use std::{
    collections::{HashSet, VecDeque},
    thread::park,
};

use crate::{intmap::IntMap, Day};

pub struct Day16 {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Object {
    Wall,
    Start,
    End,
    Empty,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Dir {
    North,
    South,
    East,
    West,
}

impl Default for Object {
    fn default() -> Self {
        Object::Empty
    }
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

fn a_star(m: &IntMap<Object>, start: (isize, isize), end: (isize, isize)) -> Option<i64> {
    let mut seen = HashSet::<((isize, isize), Dir)>::new();
    let mut q = Vec::<((isize, isize), Dir, i64)>::new();

    q.push((start, Dir::East, 0));

    loop {
        q.sort_by_key(|(_, _, cost)| -*cost);
        let Some((pos, dir, cost)) = q.pop() else {
            return None;
        };

        if seen.contains(&(pos, dir)) {
            continue;
        }
        seen.insert((pos, dir));

        if pos == end {
            return Some(cost);
        }

        for nextd in adjacent_dirs(dir) {
            q.push((pos.clone(), nextd, cost + 1000));
        }

        let dxy = delta(dir);
        let fwd = (pos.0 + dxy.0, pos.1 + dxy.1);

        if let Some(v) = m.get(fwd) {
            if v != &Object::Wall {
                q.push((fwd, dir, cost + 1));
            }
        }
    }
}

impl Day<i64> for Day16 {
    fn part1(input: &str) -> i64 {
        let (m, s, e) = parse(input);

        let depth = a_star(&m, s, e);

        depth.unwrap()
    }

    fn part2(input: &str) -> i64 {
        0
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

    #[test]
    fn part1_small() {
        assert_eq!(7036, Day16::part1(TEST_INPUT));
    }
}
