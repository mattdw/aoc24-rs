use std::collections::HashSet;

use crate::Day;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct P(i16, i16);

impl P {
    pub fn clockwise(&self) -> Self {
        P(-self.1, self.0)
    }
}

impl std::ops::Add for P {
    type Output = P;

    fn add(self, rhs: Self) -> Self::Output {
        P(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Debug, Clone)]
pub struct Day6 {
    size: P,
    guard_pos: P,
    guard_dir: P,
    walls: HashSet<P>,
    visited: HashSet<P>,
}

impl Day6 {
    fn parse(input: &str) -> Self {
        let mut x = 0;
        let mut y = 0;
        let mut x_max = 0;
        let mut guard = P(0, 0);
        let mut walls = HashSet::new();

        for c in input.trim().chars() {
            match c {
                '.' => {
                    x += 1;
                }
                '#' => {
                    walls.insert(P(x, y));
                    x += 1;
                }
                '\n' => {
                    x_max = x; // at this point we're one off the end
                    x = 0;
                    y += 1;
                }
                '^' => {
                    guard = P(x, y);
                    x += 1;
                }
                ' ' | '\t' | '\r' => {
                    ();
                }
                _ => {
                    panic!("unhandled char {c}");
                }
            }
        }

        Day6 {
            visited: HashSet::from([guard]),
            walls,
            size: P(x_max, y + 1),
            guard_pos: guard,
            guard_dir: P(0, -1),
        }
    }

    fn in_bounds(&self, p: &P) -> bool {
        0 <= p.0 && p.0 < self.size.0 && 0 <= p.1 && p.1 < self.size.1
    }

    fn next_pos(&self) -> P {
        self.guard_pos + self.guard_dir
    }

    // Allowing the option to turn off tracking gives about 3x speedup
    // for part 2.
    fn step(&mut self, track: bool) -> Option<P> {
        let next_pos = self.next_pos();

        if self.walls.contains(&next_pos) {
            self.guard_dir = self.guard_dir.clockwise();
        } else {
            if !self.in_bounds(&next_pos) {
                return None;
            }
            self.guard_pos = next_pos;
            if track {
                self.visited.insert(self.guard_pos);
            }
        }

        Some(self.guard_pos)
    }
}

impl Day<usize> for Day6 {
    fn part1(input: &str) -> usize {
        let mut d = Day6::parse(input);
        while d.step(true).is_some() {}

        d.visited.len()
    }

    fn part2(input: &str) -> usize {
        let d = Day6::parse(input);
        let mut test = d.clone();

        let mut loops = 0;

        // we run it once to get our path.
        // this takes <5ms
        while test.step(true).is_some() {}

        // then we run loop detection across all possible
        // new obstacle positions.
        for p in test.visited {
            if p == d.guard_pos {
                continue;
            }
            let mut newd = d.clone();
            newd.walls.insert(p);

            let mut hare = newd.clone();
            let mut tortoise = newd;

            loop {
                let Some(_) = hare.step(false) else {
                    break;
                };
                let Some(h) = hare.step(false) else {
                    break;
                };
                let Some(t) = tortoise.step(false) else {
                    break;
                };

                if h == t && hare.guard_dir == tortoise.guard_dir {
                    loops += 1;
                    break;
                }
            }

            //- Slower, surprisingly!
            // let mut visited_dir = HashSet::<(P, P)>::new();
            // loop {
            //     let Some(h) = hare.step() else {
            //         break;
            //     };

            //     if visited_dir.contains(&(h, hare.guard_dir)) {
            //         loops += 1;
            //         break;
            //     }

            //     visited_dir.insert((h, hare.guard_dir));
            // }
        }

        loops
    }
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str = "
    ....#.....
    .........#
    ..........
    ..#.......
    .......#..
    ..........
    .#..^.....
    ........#.
    #.........
    ......#...
    ";

    use super::*;

    #[test]
    fn parse_t() {
        let d = Day6::parse(&TEST_INPUT);

        assert_eq!(d.size, P(10, 10));
        assert_eq!(true, d.walls.contains(&P(2, 3)));
    }

    #[test]
    fn run_t_1() {
        let mut d = Day6::parse(&TEST_INPUT);

        while d.step(true).is_some() {}

        assert_eq!(41, d.visited.len());
    }

    #[test]
    fn run_t_2() {
        assert_eq!(6, Day6::part2(TEST_INPUT));
    }
}
