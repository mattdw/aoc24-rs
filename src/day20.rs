use std::collections::{BinaryHeap, HashMap};

use crate::{intmap::IntMap, Day};

#[derive(PartialEq, Eq, Default, Clone, Copy, Debug)]
enum O {
    Wall,
    Start,
    End,
    #[default]
    Empty,
}

type Co = (isize, isize);

fn manhattan(a: &Co, b: &Co) -> isize {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

#[derive(Eq, PartialEq, Clone)]
struct QNode(i32, Co, Vec<(Co, i32)>);

impl Ord for QNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for QNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Day20 {
    map: IntMap<O>,
    s: (isize, isize),
    e: (isize, isize),
}

impl Day20 {
    fn parse(input: &str) -> Day20 {
        let m = IntMap::from_raw(input, |c| match c {
            '#' => O::Wall,
            '.' => O::Empty,
            'S' => O::Start,
            'E' => O::End,
            _ => panic!("unhandled '{c}'"),
        });

        let mut start: Option<usize> = None;
        let mut end: Option<usize> = None;

        for (idx, o) in m.cells.iter().enumerate() {
            match o {
                O::Start => {
                    start = Some(idx);
                }
                O::End => {
                    end = Some(idx);
                }
                _ => {}
            };
        }

        Day20 {
            s: m.idx_to_pt(start.unwrap() as isize),
            e: m.idx_to_pt(end.unwrap() as isize),
            map: m,
        }
    }

    fn dijkstra(
        m: &IntMap<O>,
        start: &Co,
        end: &Co,
    ) -> (Option<i32>, IntMap<bool>, Vec<(Co, i32)>) {
        let mut seen = IntMap::<bool>::new(m.width, m.height);
        let mut q = BinaryHeap::<QNode>::new();
        // could replace route by storing cost in the 'seen' intmap
        q.push(QNode(0, *start, vec![(*start, 0)]));

        while let Some(QNode(cost, pos, route)) = q.pop() {
            if &pos == end {
                return (Some(cost), seen, route);
            }

            if let Some(true) = seen.get(pos) {
                continue;
            } else {
                seen.set(pos, true);
            }

            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let co = (pos.0 + dx, pos.1 + dy);
                match m.get(co) {
                    None => {}
                    Some(O::Wall) => {}
                    _ => {
                        let mut new_route = route.clone();
                        new_route.push((co, cost + 1));
                        q.push(QNode(cost + 1, co, new_route));
                    }
                }
            }
        }

        (None, seen, vec![])
    }

    fn find_cheats(input: &str, length: i32) -> Vec<(i32, i32)> {
        let m = Day20::parse(input);
        let (_, _, route) = &Day20::dijkstra(&m.map, &m.s, &m.e);

        // let mut cheats = HashMap::<(Co, Co), i32>::new();
        let mut cheats = HashMap::<i32, i32>::new();
        for (idx, &(start, start_cost)) in route.iter().enumerate() {
            for &(end, end_cost) in route.iter().skip(idx + length as usize) {
                let man_dist = manhattan(&start, &end) as i32;
                let route_diff = end_cost - start_cost;
                if man_dist <= length && man_dist < route_diff {
                    cheats
                        .entry(route_diff - man_dist)
                        .and_modify(|v| *v += 1)
                        .or_insert(1);
                }
            }
        }

        let mut vec_res: Vec<_> = cheats.into_iter().map(|(k, v)| (v, k)).collect();
        vec_res.sort_by_key(|v| v.1);

        vec_res
    }
}

impl Day<i64> for Day20 {
    fn part1(input: &str) -> i64 {
        let results = Day20::find_cheats(input, 2);

        results
            .iter()
            .filter(|&(_count, cost)| *cost >= 100)
            .map(|(count, _)| *count as i64)
            .sum::<i64>()
    }

    fn part2(input: &str) -> i64 {
        let results = Day20::find_cheats(input, 20);

        results
            .iter()
            .filter(|&(_count, cost)| *cost >= 100)
            .map(|(count, _)| *count as i64)
            .sum::<i64>()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    const TEST_INPUT: &'static str = "
        ###############
        #...#...#.....#
        #.#.#.#.#.###.#
        #S#...#.#.#...#
        #######.#.#.###
        #######.#.#...#
        #######.#.###.#
        ###..E#...#...#
        ###.#######.###
        #...###...#...#
        #.#####.#.###.#
        #.#...#.#.#...#
        #.#.#.#.#.#.###
        #...#...#...###
        ###############
    ";

    #[test]
    fn p1_t() {
        assert_eq!(
            vec![
                (14, 2),
                (14, 4),
                (2, 6),
                (4, 8),
                (2, 10),
                (3, 12),
                (1, 20),
                (1, 36),
                (1, 38),
                (1, 40),
                (1, 64)
            ],
            Day20::find_cheats(TEST_INPUT, 2)
        )
    }
}
