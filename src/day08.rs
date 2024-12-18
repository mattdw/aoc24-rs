use std::collections::HashMap;

use crate::Day;

pub struct Day8 {
    map: HashMap<char, Vec<(i32, i32)>>,
    width: i32,
    height: i32,
}

fn parse(input: &str) -> Day8 {
    let mut out: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    let mut width: usize = 0;
    let mut height: usize = 0;
    for (y, line) in input.trim().lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            width = x + 1;
            if c == '.' {
                continue;
            }

            // if !out.contains_key(&c) {
            //     out.insert(c, vec![]);
            // }
            // let v = out.get_mut(&c).expect("we just inserted it");

            // could also .or_insert_with(|| vec![]);
            let v = out.entry(c).or_default();
            v.push((x as i32, y as i32));
        }
        height = y + 1;
    }

    // dbg!(&out);

    Day8 {
        map: out,
        height: height as i32,
        width: width as i32,
    }
}

fn cartesian_product(locs: &[(i32, i32)]) -> Vec<((i32, i32), (i32, i32))> {
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

fn antinode_pos(left: (i32, i32), right: (i32, i32)) -> (i32, i32) {
    let dx = right.0 - left.0;
    let dy = right.1 - left.1;

    (right.0 + dx, right.1 + dy)
}

fn antinode_positions_part2(d: &Day8, left: (i32, i32), right: (i32, i32)) -> Vec<(i32, i32)> {
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

fn in_bounds(d: &Day8, pt: &(i32, i32)) -> bool {
    pt.0 >= 0 && pt.0 < d.width && pt.1 >= 0 && pt.1 < d.height
}

fn gcd(a: i32, b: i32) -> i32 {
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
fn print_locs(d: &Day8, map: &HashMap<(i32, i32), char>) {
    for y in 0..d.height {
        for x in 0..d.width {
            print!("{}", map.get(&(x, y)).unwrap_or(&'.'));
        }
        println!();
    }
}

impl Day<i32> for Day8 {
    fn part1(input: &str) -> i32 {
        let info = parse(input);

        let mut antinodes = HashMap::<(i32, i32), char>::new();
        for (freq, locs) in info.map.iter() {
            for (left, right) in cartesian_product(locs) {
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

        antinodes.keys().count() as i32
    }

    fn part2(input: &str) -> i32 {
        let info = parse(input);

        let mut antinodes = HashMap::<(i32, i32), char>::new();

        for (freq, locs) in info.map.iter() {
            for (left, right) in cartesian_product(locs) {
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

        antinodes.keys().count() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "
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
