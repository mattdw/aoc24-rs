use std::collections::HashMap;

use crate::Day;

pub struct Day8 {
    map: HashMap<char, Vec<(isize, isize)>>,
    width: isize,
    height: isize,
}

fn parse(input: &str) -> Day8 {
    let mut out = HashMap::new();

    let mut width: usize = 0;
    let mut height: usize = 0;
    for (y, line) in input.trim().lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            width = x + 1;
            if c == '.' {
                continue;
            }
            if !out.contains_key(&c) {
                out.insert(c, vec![]);
            }
            let v = out.get_mut(&c).expect("we just inserted it");
            v.push((x as isize, y as isize));
        }
        height = y + 1;
    }

    // dbg!(&out);

    Day8 {
        map: out,
        height: height as isize,
        width: width as isize,
    }
}

fn all_pairs(locs: &Vec<(isize, isize)>) -> Vec<((isize, isize), (isize, isize))> {
    let mut outs = vec![];
    for left in locs.iter() {
        for right in locs.iter() {
            if right == left {
                continue;
            }
            outs.push((*left, *right));
        }
    }
    outs
}

fn antinode_pos(left: (isize, isize), right: (isize, isize)) -> (isize, isize) {
    let dx = right.0 - left.0;
    let dy = right.1 - left.1;

    (right.0 + dx, right.1 + dy)
}

fn antinode_positions_part2(
    d: &Day8,
    left: (isize, isize),
    right: (isize, isize),
) -> Vec<(isize, isize)> {
    let mut dx = right.0 - left.0;
    let mut dy = right.1 - left.1;

    let dd = gcd(dx, dy);

    dx /= dd;
    dy /= dd;

    let mut outs = vec![];

    let mut curr_x = left.0 + dx;
    let mut curr_y = left.1 + dy;
    while in_bounds(d, &(curr_x, curr_y)) {
        // println!("from {:?}->{:?} adding {:?}", left, right, (curr_x, curr_y));
        outs.push((curr_x, curr_y));
        curr_x += dx;
        curr_y += dy;
    }

    outs
}

fn in_bounds(d: &Day8, pt: &(isize, isize)) -> bool {
    pt.0 >= 0 && pt.0 < d.width && pt.1 >= 0 && pt.1 < d.height
}

fn gcd(a: isize, b: isize) -> isize {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a.abs()
}

#[allow(dead_code)]
fn print_locs(d: &Day8, map: &HashMap<(isize, isize), char>) {
    for y in 0..d.height {
        for x in 0..d.width {
            print!("{}", map.get(&(x, y)).unwrap_or(&'.'));
        }
        println!();
    }
}

impl Day<isize> for Day8 {
    fn part1(input: &str) -> isize {
        let info = parse(input);

        let mut antinodes = HashMap::<(isize, isize), char>::new();
        for (freq, locs) in info.map.iter() {
            for (left, right) in all_pairs(locs) {
                // println!("{:?} -> {:?}", left, right);
                let dest = antinode_pos(left, right);
                if in_bounds(&info, &dest) {
                    antinodes.insert(dest, *freq);
                }
            }
        }

        // print_locs(&info, &antinodes);
        // dbg!(&antinodes);
        // println!("w {} h {}", &info.width, &info.height);

        antinodes.keys().count() as isize
    }

    fn part2(input: &str) -> isize {
        let info = parse(input);

        let mut antinodes = HashMap::<(isize, isize), char>::new();

        for (freq, locs) in info.map.iter() {
            for (left, right) in all_pairs(locs) {
                // println!("{:?} -> {:?}", left, right);
                let dests = antinode_positions_part2(&info, left, right);
                for dest in dests {
                    antinodes.insert(dest, *freq);
                }
            }
        }

        // print_locs(&info, &antinodes);
        // dbg!(&antinodes);
        // println!("w {} h {}", &info.width, &info.height);

        antinodes.keys().count() as isize
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &'static str = "
    ............
    ........0...
    .....0......
    .......0....
    ....0.......
    ......A.....
    ............
    ............
    ........A...
    .........A..
    ............
    ............
    ";

    #[test]
    fn part1() {
        assert_eq!(Day8::part1(TEST_INPUT), 14);
    }

    #[test]
    fn part2() {
        assert_eq!(Day8::part2(TEST_INPUT), 34);
    }
}
