use crate::Day;

pub struct Day10 {}

type Pt = (i64, i64);

struct Map {
    width: i64,
    height: i64,
    cells: Vec<i64>,
}

impl Map {
    fn pt_to_idx(&self, p: Pt) -> usize {
        (p.1 * self.width + p.0) as usize
    }

    // fn idx_to_pt(&self, idx: usize) -> Pt {
    //     let x = idx as i64 % self.width;
    //     let y = idx as i64 / self.height;

    //     (x, y)
    // }

    fn in_bounds(&self, p: Pt) -> bool {
        0 <= p.0 && p.0 < self.width && 0 <= p.1 && p.1 < self.height
    }

    fn get(&self, p: Pt) -> Option<i64> {
        if !self.in_bounds(p) {
            return None;
        }
        Some(self.cells[self.pt_to_idx(p)])
    }

    // fn put(&mut self, p: Pt, val: i64) -> Option<i64> {
    //     if !self.in_bounds(p) {
    //         return None;
    //     }
    //     let id = self.pt_to_idx(p);
    //     self.cells[id] = val;
    //     Some(val)
    // }

    fn all_pts(&self) -> Vec<Pt> {
        let mut outs = vec![];
        for x in 0..self.width {
            for y in 0..self.width {
                outs.push((x, y))
            }
        }
        outs
    }
}

fn parse_map(input: &str) -> Map {
    let mut width: i64 = 0;

    let cells: Vec<i64> = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            width = ((i as i64) + 1).max(width);
            line.trim()
                .chars()
                .map(|v| v.to_digit(10).expect("not a digit") as i64)
        })
        .collect();

    let height = cells.len() as i64 / width;
    Map {
        width,
        height,
        cells,
    }
}

fn reachable(m: &Map, zero: Pt, nine: Pt) -> bool {
    let mut open = std::collections::HashSet::new();
    open.insert(zero);

    while open.len() > 0 {
        let curr = *open.iter().take(1).collect::<Vec<_>>()[0];
        open.remove(&curr);
        if curr == nine {
            return true;
        }
        let here = m.get(curr).unwrap();

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let co = (curr.0 + dx, curr.1 + dy);
            if !m.in_bounds(co) {
                continue;
            }
            if m.get(co) == Some(here + 1) {
                open.insert(co);
            }
        }
    }

    false
}

// Annoyingly I got too clever with this and accidentally solved part 2 without
// realising it. Then deleted it and implemented degenerate A* as part 1
// actually required, only to realise I still wanted the old approach but it was
// no longer in my undo history.

// Anyway part 2 is quite a satisfying spread/contagion approach.

fn solve_a(m: &Map) -> i64 {
    let zeroes = m
        .all_pts()
        .iter()
        .map(|&pt| (pt, m.get(pt).unwrap()))
        .filter(|(_, v)| *v == 0)
        .map(|(a, _)| a)
        .collect::<Vec<_>>();
    let nines = m
        .all_pts()
        .iter()
        .map(|&pt| (pt, m.get(pt).unwrap()))
        .filter(|(_, v)| *v == 9)
        .map(|(a, _)| a)
        .collect::<Vec<_>>();

    // println!("{:?}\n{:?}", zeroes, nines);

    let mut count = 0;
    for z in &zeroes {
        for n in &nines {
            if reachable(m, *z, *n) {
                count += 1;
            }
        }
    }

    count
}

fn solve_b(m: &Map) -> i64 {
    let mut counts: Vec<_> = m
        .cells
        .iter()
        .map(|&v| if v == 9 { 1 } else { 0 })
        .collect();

    for i in (1..=9).rev() {
        for p in m.all_pts() {
            if m.get(p).unwrap() != (i - 1) {
                continue;
            }
            counts[m.pt_to_idx(p)] = [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .iter()
                .map(|(dx, dy)| {
                    let co = (p.0 + dx, p.1 + dy);
                    if m.get(co) == Some(i) {
                        let idx = m.pt_to_idx(co);
                        counts[idx]
                    } else {
                        0
                    }
                })
                .sum();
        }
    }

    m.all_pts()
        .into_iter()
        .filter(|&p| m.get(p) == Some(0))
        .map(|p| counts[m.pt_to_idx(p)])
        .sum()
}

impl Day<i64> for Day10 {
    fn part1(input: &str) -> i64 {
        solve_a(&parse_map(input))
    }

    fn part2(input: &str) -> i64 {
        solve_b(&parse_map(input))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &'static str = "
        89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732
        ";

    #[test]
    fn test_part1() {
        assert_eq!(36, Day10::part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(81, Day10::part2(TEST_INPUT));
    }
}
