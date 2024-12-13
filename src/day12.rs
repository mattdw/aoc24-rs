use std::collections::{HashMap, HashSet, VecDeque};

use crate::Day;

#[derive(Hash, Debug, Copy, Clone, Eq, PartialEq)]
struct Co(i64, i64);

struct Map {
    #[allow(dead_code)]
    data: HashMap<Co, i64>,
    regions: HashMap<i64, HashSet<Co>>,
    fences: HashMap<Co, i64>,
    #[allow(dead_code)]
    width: i64,
    #[allow(dead_code)]
    height: i64,
}

fn parse(input: &str) -> Map {
    let mut charmap = HashMap::<Co, char>::new();
    let mut regionmap = HashMap::<Co, i64>::new();
    let mut revmap = HashMap::<i64, HashSet<Co>>::new();
    let mut region_id: i64 = 0;
    let mut fencemap = HashMap::<Co, i64>::new();

    let mut next_id = || {
        let val = region_id;
        region_id += 1;

        val
    };

    let mut width: i64 = 0;
    let mut height: i64 = 0;
    for (y, line) in input.trim().lines().enumerate() {
        height = height.max(y as i64 + 1);
        for (x, c) in line.trim().chars().enumerate() {
            width = width.max(x as i64 + 1);
            charmap.insert(Co(x as i64, y as i64), c);
        }
    }

    let mut q = VecDeque::new();
    let mut seen = HashSet::new();
    q.push_back(Co(0, 0));
    loop {
        let Some(co) = q.pop_front() else {
            break;
        };

        if seen.contains(&co) {
            continue;
        } else {
            seen.insert(co);
        }

        let char = charmap.get(&co).expect("shouldn't be queueing empty cells");
        let mut region_cos = HashSet::new();
        region_cos.insert(co);
        let mut region_q = VecDeque::new();
        let mut region_seen = HashSet::new();
        region_q.push_back(co);

        loop {
            let Some(co) = region_q.pop_front() else {
                break;
            };

            if region_seen.contains(&co) {
                continue;
            } else {
                region_seen.insert(co);
                seen.insert(co);
            }
            let mut fences = 4;
            for (dx, dy) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
                let co2 = Co(co.0 + dx, co.1 + dy);
                if let Some(other_char) = charmap.get(&co2) {
                    if other_char == char {
                        region_cos.insert(co2);
                        region_q.push_back(co2);
                        fences -= 1;
                    } else {
                        q.push_back(co2);
                    }
                }
            }
            fencemap.insert(co, fences);
        }

        revmap.insert(next_id(), region_cos);
    }

    for (r, rs) in revmap.iter() {
        for co in rs.iter() {
            regionmap.insert(*co, *r);
        }
    }

    Map {
        data: regionmap,
        regions: revmap,
        fences: fencemap,
        width,
        height,
    }
}

fn score1(map: &Map) -> i64 {
    map.regions
        .values()
        .map(|cos| {
            cos.iter()
                .map(|co| map.fences.get(co).unwrap())
                .sum::<i64>()
                * cos.len() as i64
        })
        .sum()
}

fn count_internal_corners(map: &Map, co: &Co, id: i64) -> i64 {
    [(-1, -1), (-1, 1), (1, 1), (1, -1)]
        .iter()
        .map(|(dx, dy)| {
            let side_offs = [(*dx, 0), (0, *dy)];
            let sides = side_offs
                .iter()
                .map(|(dx, dy)| map.data.get(&Co(co.0 + *dx, co.1 + *dy)))
                .collect::<Vec<_>>();
            // eprintln!("{sides:?}");
            if sides.iter().any(|&v| v != Some(&id)) {
                return 0;
            }
            let otherco = Co(co.0 + dx, co.1 + dy);
            if let Some(other_id) = map.data.get(&otherco) {
                if *other_id != id {
                    // eprintln!("{otherco:?} is internal corner to {co:?}");
                    1
                } else {
                    0
                }
            } else {
                // out-of-bounds is never an internal corner
                0
            }
        })
        .sum()
}

fn fences_opposite(map: &Map, co: &Co, id: i64) -> bool {
    // eprintln!("checking opposite fences for {co:?}");
    let sides = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let ids = sides
        .iter()
        .map(|(dx, dy)| map.data.get(&Co(co.0 + dx, co.1 + dy)))
        .collect::<Vec<_>>();

    let me = Some(&id);
    (ids[0] == me && ids[1] == me) || (ids[2] == me && ids[3] == me)
}
fn count_corners(map: &Map, co: &Co, id: i64) -> i64 {
    let fences = map.fences.get(co).expect("has a score");
    match fences {
        4 => 4,
        3 => 2,
        2 => {
            if dbg!(fences_opposite(map, co, id)) {
                0
            } else {
                1 + count_internal_corners(map, co, id)
            }
        }
        1 => count_internal_corners(map, co, id),
        0 => count_internal_corners(map, co, id),
        _ => unreachable!("{fences} score??"),
    }
}

fn score2(map: &Map) -> i64 {
    map.regions
        .iter()
        .map(|(id, cos)| {
            let area = cos.len() as i64;
            let corner_count: i64 = cos.iter().map(|co| count_corners(map, co, *id)).sum();
            // eprintln!("ID {id}: {area} * {corner_count}");
            area * corner_count
        })
        .sum()
}

pub struct Day12 {}

impl Day<i64> for Day12 {
    fn part1(input: &str) -> i64 {
        score1(&parse(input))
    }

    fn part2(input: &str) -> i64 {
        score2(&parse(input))
    }
}

#[cfg(test)]
mod test {

    const TEST_INPUT: &'static str = "
    AAAA
    BBCD
    BBCC
    EEEC
    ";

    const TEST_INPUT_LARGER: &'static str = "
    RRRRIICCFF
    RRRRIICCCF
    VVRRRCCFFF
    VVRCCCJFFF
    VVVVCJJCFE
    VVIVCCJJEE
    VVIIICJJEE
    MIIIIIJJEE
    MIIISIJEEE
    MMMISSJEEE";

    #[test]
    fn parse() {
        let r = super::parse(TEST_INPUT);
        assert_eq!(r.width, 4);
        assert_eq!(r.height, 4);
    }

    #[test]
    fn parse_regions() {
        let r = super::parse(TEST_INPUT);
        assert_eq!(5, r.regions.len());
    }

    #[test]
    fn parse_larger() {
        let r = super::parse(TEST_INPUT_LARGER);
        assert_eq!(11, r.regions.len());
        assert_eq!(10, r.width);
    }

    #[test]
    fn score_small() {
        let r = super::parse(TEST_INPUT);
        assert_eq!(140, super::score1(&r));
    }

    #[test]
    fn score_larger() {
        let r = super::parse(TEST_INPUT_LARGER);
        assert_eq!(1930, super::score1(&r));
    }

    #[test]
    fn score2_small() {
        let r = super::parse(TEST_INPUT);
        assert_eq!(80, super::score2(&r));
    }

    #[test]
    fn score2() {
        let r = super::parse(TEST_INPUT_LARGER);
        assert_eq!(1206, super::score2(&r));
    }

    #[test]
    fn score2_tiny() {
        let p = super::parse(
            "
        .C
        CC
        C.
        ",
        );

        assert_eq!(32 + 4 + 4, super::score2(&p));
    }

    #[test]
    fn score2_block() {
        let p = super::parse(
            "
        EEE
        ",
        );

        assert_eq!(12, super::score2(&p));
    }
}
