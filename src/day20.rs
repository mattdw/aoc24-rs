use std::{
    collections::{BinaryHeap, HashMap},
    ops::RangeBounds,
};

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

#[derive(Eq, PartialEq, Clone, Copy)]
struct QNode(i32, Co, Option<Co>, Option<Co>);

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

    fn wall_positions(m: &IntMap<O>, filter: Option<&IntMap<bool>>) -> Vec<Co> {
        m.cells
            .iter()
            .enumerate()
            .filter(|(idx, _v)| {
                filter.is_none_or(|f| *f.get(f.idx_to_pt(*idx as isize)).unwrap_or(&false))
            })
            .filter(|(_idx, &v)| v == O::Wall)
            .map(|(idx, _v)| m.idx_to_pt(idx as isize))
            .collect()
    }

    fn dijkstra(
        m: &IntMap<O>,
        start: &Co,
        end: &Co,
        disable_collisions_at: i32,
        disable_collisions_for: i32,
    ) -> Vec<(Option<i32>, IntMap<bool>, Option<Co>, Option<Co>)> {
        let mut seen = IntMap::<bool>::new(m.width, m.height);
        let mut q = BinaryHeap::<QNode>::new();
        q.push(QNode(0, *start, None, None));

        let disable_collisions_until = disable_collisions_at + disable_collisions_for;
        // dbg!(disable_collisions_until);

        let mut routes = Vec::<(Option<i32>, IntMap<bool>, Option<Co>, Option<Co>)>::new();

        while let Some(QNode(cost, pos, walls_on_pos, walls_off_pos)) = q.pop() {
            if &pos == end {
                routes.push((
                    Some(cost),
                    seen.clone(),
                    walls_on_pos,
                    walls_off_pos.or(Some(*end)),
                ));
                continue;
            }

            // dbg!(cost);

            let w_on_p = if cost == (disable_collisions_at + 0) {
                // println!("disabling collisions at {}", cost);
                Some(pos)
            } else {
                walls_on_pos
            };

            let w_off_p = if cost == disable_collisions_until {
                // println!("enabing collisions at {}", cost);
                Some(pos)
            } else {
                walls_off_pos
            };

            if let Some(true) = seen.get(pos) {
                continue;
            } else {
                seen.set(pos, true);
            }

            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let co = (pos.0 + dx, pos.1 + dy);
                match m.get(co) {
                    None => {}
                    Some(O::Wall) => {
                        if (disable_collisions_at..disable_collisions_until).contains(&(cost + 1)) {
                            q.push(QNode(cost + 1, co, w_on_p, w_off_p));
                        } else {
                            seen.set(co, true);
                        }
                    }
                    _ => {
                        q.push(QNode(cost + 1, co, w_on_p, w_off_p));
                    }
                }
            }
        }

        routes
    }

    fn part2_results(input: &str) -> HashMap<((isize, isize), (isize, isize)), (i32, i32)> {
        let m = Day20::parse(input);
        let first_run = Day20::dijkstra(&m.map, &m.s, &m.e, i32::MAX - 1, 0);
        for (cost, _, _, _) in &first_run {
            println!("first run: {cost:?}");
        }
        let first_run_cost = first_run[0].0.unwrap();

        // let near_walls = Day20::wall_positions(&m.map, Some(&first_run.1));

        let mut results = HashMap::<(Co, Co), (i32, i32)>::new();
        for i in 0..9388 {
            // let mut new_map = m.map.clone();
            // new_map.set(w, O::Empty);

            for r in Day20::dijkstra(&m.map, &m.s, &m.e, i, 20) {
                let (cost, _, a, b) = r;
                println!("? {cost:?}, {a:?}-{b:?}");

                if a.is_none() || b.is_none() {
                    continue;
                }

                // println!("{:?}", new_res.0);
                results
                    .entry((a.unwrap(), b.unwrap()))
                    .and_modify(|cost_count| {
                        *cost_count = (
                            cost_count.0.max(first_run_cost - cost.unwrap()),
                            cost_count.1 + 1,
                        );
                    })
                    .or_insert((first_run_cost - cost.unwrap(), 1));
            }
        }

        dbg!(&results);
        results
    }
}

impl Day<i64> for Day20 {
    fn part1(input: &str) -> i64 {
        let m = Day20::parse(input);
        let first_run = &Day20::dijkstra(&m.map, &m.s, &m.e, i32::MAX - 1, 0)[0];

        let near_walls = Day20::wall_positions(&m.map, Some(&first_run.1));

        let mut results = HashMap::<i32, i32>::new();

        for w in near_walls {
            let mut new_map = m.map.clone();
            new_map.set(w, O::Empty);

            let new_res = &Day20::dijkstra(&new_map, &m.s, &m.e, i32::MAX - 1, 0)[0];

            println!("{:?}", new_res.0);
            results
                .entry(&first_run.0.unwrap() - new_res.0.unwrap())
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }

        results
            .iter()
            .filter(|(&k, _)| k >= 100)
            .map(|(k, &v)| v as i64)
            .sum::<i64>()
    }

    fn part2(input: &str) -> i64 {
        let results = Day20::part2_results(input);

        results
            .iter()
            .filter(|(_, (saved, _))| *saved >= 100)
            .count() as i64
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

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
    fn dijkstra_t() {
        let m = Day20::parse(TEST_INPUT);
        let first_run = Day20::dijkstra(&m.map, &m.s, &m.e, i32::MAX - 1, 0);

        assert_eq!(Some(84), first_run[0].0);

        let near_walls = Day20::wall_positions(&m.map, Some(&first_run[0].1));

        let mut results = HashMap::<i32, i32>::new();

        for w in near_walls {
            let mut new_map = m.map.clone();
            new_map.set(w, O::Empty);

            let new_res = &Day20::dijkstra(&new_map, &m.s, &m.e, i32::MAX - 1, 0)[0];

            // println!("{:?}", new_res.0);
            results
                .entry(&first_run[0].0.unwrap() - new_res.0.unwrap())
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }

        // dbg!(results);
    }

    #[test]
    fn p2_t() {
        let results = Day20::part2_results(TEST_INPUT);

        dbg!(&results);

        assert_eq!(22, results.values().filter(|(c, _)| *c == 72).count())

        // results
        // .iter()
        // .filter(|(_, (saved, _))| saved >= 100)
        // .count() as i64
    }
}
