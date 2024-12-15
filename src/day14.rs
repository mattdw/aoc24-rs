use crate::Day;

use nalgebra::Vector2;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Robot {
    p: Vector2<i64>,
    v: Vector2<i64>,
}

fn parse(input: &str) -> Vec<Robot> {
    let pat = Regex::new(r"p=(?<px>[-\d]+),(?<py>[-\d]+) v=(?<vx>[-\d]+),(?<vy>[-\d]+)")
        .expect("regex error");

    pat.captures_iter(input)
        .map(|m| Robot {
            p: Vector2::new(m["px"].parse().unwrap(), m["py"].parse().unwrap()),
            v: Vector2::new(m["vx"].parse().unwrap(), m["vy"].parse().unwrap()),
        })
        .collect()
}

fn step(r: Robot, world_size: (i64, i64), n: i64) -> Robot {
    let mut r = r;

    for _ in 0..n {
        r.p += r.v;
        if r.p.x < 0 {
            r.p.x += world_size.0;
        }
        if r.p.y < 0 {
            r.p.y += world_size.1;
        }
        if r.p.x >= world_size.0 {
            r.p.x -= world_size.0;
        }
        if r.p.y >= world_size.1 {
            r.p.y -= world_size.1;
        }
    }

    r
}

fn quadrant(r: &Robot, world_size: (i64, i64)) -> Option<i64> {
    let qx = if r.p.x < (world_size.0 / 2) {
        Some(0)
    } else if r.p.x > (world_size.0 / 2) {
        Some(1)
    } else {
        None
    };

    let qy = if r.p.y < (world_size.1 / 2) {
        Some(0)
    } else if r.p.y > (world_size.1 / 2) {
        Some(1)
    } else {
        None
    };

    qx.and_then(|vx| qy.map(|vy| vy * 2 + vx))
}

fn has_span(rs: &Vec<Robot>, world_size: (i64, i64), threshold: i64) -> bool {
    use std::collections::HashSet;
    let mut map = HashSet::<Vector2<i64>>::new();
    for r in rs {
        map.insert(r.p);
    }

    let mut longest_span = 0;
    let mut current_span = 0;

    for y in 0..world_size.1 {
        for x in 0..world_size.0 {
            let c = if map.contains(&Vector2::new(x, y)) {
                current_span += 1;
                if current_span > threshold {
                    return true;
                }
                '#'
            } else {
                longest_span = longest_span.max(current_span);
                current_span = 0;
                '.'
            };
        }
    }

    false
}

fn print_map(rs: &Vec<Robot>, world_size: (i64, i64), output: bool) -> i64 {
    use std::collections::HashSet;
    let mut map = HashSet::<Vector2<i64>>::new();
    for r in rs {
        map.insert(r.p);
    }

    let mut longest_span = 0;
    let mut current_span = 0;

    for y in 0..world_size.1 {
        for x in 0..world_size.0 {
            let c = if map.contains(&Vector2::new(x, y)) {
                current_span += 1;
                '#'
            } else {
                longest_span = longest_span.max(current_span);
                current_span = 0;
                '.'
            };
            if output {
                print!("{}", c);
            }
        }
        if output {
            println!();
        }
    }

    longest_span
}

pub struct Day14 {}

impl Day<i64> for Day14 {
    fn part1(input: &str) -> i64 {
        let rs = parse(input);
        let world_size = (101, 103);

        let rs = rs.iter().map(|&r| step(r, world_size, 100));
        let mut counts = [0; 4];
        for r in rs {
            if let Some(q) = quadrant(&r, world_size) {
                counts[q as usize] += 1;
            }
        }

        counts[0] * counts[1] * counts[2] * counts[3]
    }

    fn part2(input: &str) -> i64 {
        let mut rs = parse(input);
        let world_size = (101, 103);

        let mut steps = 0;
        loop {
            steps += 1;
            let rs_ = rs.iter().map(|&r| step(r, world_size, 1)).collect();
            let mut counts = [0; 4];
            for r in &rs_ {
                if let Some(q) = quadrant(r, world_size) {
                    counts[q as usize] += 1;
                } else {
                }
            }

            println!("tree on {}", steps);
            if has_span(&rs_, world_size, 10) {
                print_map(&rs_, world_size, true);
                break;
            }

            rs = rs_;
        }

        steps
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &'static str = "p=0,4 v=3,-3
        p=6,3 v=-1,-3
        p=10,3 v=-1,2
        p=2,0 v=2,-1
        p=0,0 v=1,3
        p=3,0 v=-2,-2
        p=7,6 v=-1,-3
        p=3,0 v=-1,-2
        p=9,3 v=2,3
        p=7,3 v=-1,2
        p=2,4 v=2,-3
        p=9,5 v=-3,-3";

    #[test]
    fn parse_t() {
        let rs = super::parse(&TEST_INPUT);
        assert_eq!(rs.len(), 12);
    }

    #[test]
    fn step_t() {
        let r = Robot {
            p: Vector2::new(2, 4),
            v: Vector2::new(2, -3),
        };
        let r = step(r, (11, 7), 5);

        assert_eq!(r.p, Vector2::new(1, 3));
    }

    #[test]
    fn quadrant_t() {
        let mut r = Robot {
            p: Vector2::new(2, 4),
            v: Vector2::new(2, -3),
        };
        assert_eq!(quadrant(&r, (11, 7)), Some(2));

        r.p.y = 0;
        assert_eq!(quadrant(&r, (11, 7)), Some(0));

        r.p.x = 5;
        assert_eq!(quadrant(&r, (11, 7)), None);
    }
}
