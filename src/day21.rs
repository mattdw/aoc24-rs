/*

--- Part Two ---

Just as the missing Historian is released, The Historians realize that a second member of their search party has also been missing this entire time!

A quick life-form scan reveals the Historian is also trapped in a locked area of the ship. Due to a variety of hazards, robots are once again dispatched, forming another chain of remote control keypads managing robotic-arm-wielding robots.

This time, many more robots are involved. In summary, there are the following keypads:

One directional keypad that you are using.
25 directional keypads that robots are using.
One numeric keypad (on a door) that a robot is using.
The keypads form a chain, just like before: your directional keypad controls a robot which is typing on a directional keypad which controls a robot which is typing on a directional keypad... and so on, ending with the robot which is typing on the numeric keypad.

The door codes are the same this time around; only the number of robots and directional keypads has changed.

Find the fewest number of button presses you'll need to perform in order to cause the robot in front of the door to type each code. What is the sum of the complexities of the five codes on your list?

*/

use std::{collections::HashMap, io::Read};

use crate::Day;
pub struct Day21 {}

// we can just treat x then y deltas as movement instructions
// so e.g. (0,0) -> (3,1) = >>>v

const NUMPAD: &'static str = "
    789
    456
    123
    _0A
    ";

const ARROWS: &'static str = "
    _^A
    <v>
    ";

type PadMap = [(i8, i8); 128];

fn parse_map(s: &str) -> PadMap {
    let mut arr = [(0, 0); 128];
    let mut x = 0;
    let mut y = 0;
    for c in s.trim().bytes() {
        match c {
            b' ' | b'\t' | b'\r' => {}
            b'\n' => {
                x = 0;
                y += 1;
            }
            c => {
                arr[c as usize] = (x, y);
                x += 1;
            }
        }
    }

    arr
}

const ups: [u8; 16] = [b'^'; 16];
const downs: [u8; 16] = [b'v'; 16];
const lefts: [u8; 16] = [b'<'; 16];
const rights: [u8; 16] = [b'>'; 16];

// fn unpack_delta(co: (i8, i8), x_first: bool) -> Vec<u8> {
//     // while x != 0 {
//     //     if x < 0 {
//     //         xs.push(b'<');
//     //         x += 1;
//     //     } else {
//     //         xs.push(b'>');
//     //         x -= 1;
//     //     }
//     // }

//     // while y != 0 {
//     //     if y < 0 {
//     //         ys.push(b'^');
//     //         y += 1;
//     //     } else {
//     //         ys.push(b'v');
//     //         y -= 1;
//     //     }
//     // }

//     if x_first {
//         [xs, ys].concat()
//     } else {
//         [ys, xs].concat()
//     }
// }

// favour ^> v> <^ <v
fn direction_xy(curr: &(i8, i8), dest: &(i8, i8), empty: &(i8, i8)) -> bool {
    let favour_left = (dest.0 - curr.0) < 0;
    // both empties are on x==0
    if curr.0 > 0 && dest.0 > 0 {
        return favour_left;
    }

    let e = empty;

    // starting on same row and finishing on same column
    // gotta do y first
    if curr.1 == e.1 && dest.0 == e.0 {
        return false;
    }

    // starting on same column and finishing on same row -
    // gotta do x first
    if curr.0 == e.0 && dest.1 == e.1 {
        return true;
    }

    // moving left
    if dest.0 < curr.0 {
        return true;
    }

    if dest.0 > curr.0 {
        return false;
    }

    // moving up
    if dest.1 < curr.1 {
        return false;
    }

    false
}

fn plan(target: &[u8], pad: &PadMap) -> Vec<u8> {
    let A_pos = pad[b'A' as usize];
    let empty = pad[b'_' as usize];

    let mut cache = vec![Vec::<u8>::new(); 128 * 128];

    let mut curr = A_pos;
    let mut curr_b = b'A';
    let mut out = Vec::<u8>::new();
    for &b in target.iter() {
        if let Some(v) = cache.get(curr_b as usize * 128 + b as usize) {
            if !v.is_empty() {
                out.extend(v);
                curr = pad[b as usize];
                curr_b = b;
                continue;
            }
        }

        let c = pad[b as usize];
        // navigate to each button
        let delta = (c.0 - curr.0, c.1 - curr.1);

        let x_first = direction_xy(&curr, &c, &empty);

        let (x, y) = delta;
        let xs = &(if x < 0 { lefts } else { rights })[0..(x.abs() as usize)];
        let ys = &(if y < 0 { ups } else { downs })[0..(y.abs() as usize)];

        let mut me = Vec::new();

        // let ds = unpack_delta(delta, x_first);
        if x_first {
            me.extend(xs);
            me.extend(ys);
        } else {
            me.extend(ys);
            me.extend(xs);
        }
        // push each button
        me.push(b'A');
        cache[curr_b as usize * 128 + b as usize] = me.clone();

        out.extend(me);
        curr = c;
        curr_b = b;
    }

    out
}

fn plan_len(
    from_char: u8,
    to_char: u8,
    // target: &[u8],
    pad_arrows: &PadMap,
    pad_nums: &PadMap,
    depth: usize,
    cache: &mut HashMap<(u8, u8, usize), usize>,
) -> usize {
    let pad = if depth == 0 { pad_nums } else { pad_arrows };
    let A_pos = pad[b'A' as usize];
    let empty = pad[b'_' as usize];

    // let mut cache = vec![Vec::<u8>::new(); 128 * 128];

    if let Some(c) = cache.get(&(from_char, to_char, depth)) {
        return *c;
    }

    let mut curr = pad[from_char as usize];
    let mut curr_b = from_char;
    let mut out = Vec::<u8>::new();

    let c = pad[to_char as usize];
    // navigate to each button
    let delta = (c.0 - curr.0, c.1 - curr.1);

    let x_first = direction_xy(&curr, &c, &empty);

    let (x, y) = delta;
    let xs = &(if x < 0 { lefts } else { rights })[0..(x.abs() as usize)];
    let ys = &(if y < 0 { ups } else { downs })[0..(y.abs() as usize)];

    let mut me = Vec::new();

    // let ds = unpack_delta(delta, x_first);
    if x_first {
        me.extend(xs);
        me.extend(ys);
    } else {
        me.extend(ys);
        me.extend(xs);
    }
    // push each button
    me.push(b'A');

    cache.insert((from_char, to_char, depth), me.len());

    me.len();

    out.len()
}

impl Day<isize> for Day21 {
    fn part1(input: &str) -> isize {
        let n = parse_map(NUMPAD);
        let a = parse_map(ARROWS);

        let mut complexity = 0;
        for p0 in input.split_whitespace() {
            let p1 = plan(p0.as_bytes(), &n);
            let p2 = plan(&p1, &a);
            let p3 = plan(&p2, &a);

            complexity += p0[0..3].parse::<isize>().unwrap() * p3.len() as isize;
        }

        complexity
    }

    fn part2(input: &str) -> isize {
        let n = parse_map(NUMPAD);
        let a = parse_map(ARROWS);

        let mut complexity = 0;
        for p0 in input.split_whitespace() {
            let p1 = plan(p0.as_bytes(), &n);

            let mut curr = p1;
            for _i in 0..25 {
                curr = plan(&curr, &a);
                // println!("{_i}: {:?}", &curr.len());
                println!("{_i}");
            }

            let p3 = plan(&curr, &a);

            complexity += p0[0..3].parse::<isize>().unwrap() * p3.len() as isize;
        }

        complexity
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn keymap_parse() {
        let a = parse_map(NUMPAD);
        assert_eq!((1, 2), a[b'2' as usize]);
    }

    #[test]
    fn dirmap_parse() {
        let a = parse_map(ARROWS);
        assert_eq!((2, 0), a[b'A' as usize]);
    }

    #[test]
    fn type_029A() {
        let m = parse_map(NUMPAD);
        let p = plan("029A".as_bytes(), &m);
        assert_eq!("<A^A>^^AvvvA".len(), p.len());
    }

    #[test]
    fn direct_029A_1() {
        let n = parse_map(NUMPAD);
        let a = parse_map(ARROWS);

        let p1 = plan("029A".as_bytes(), &n);
        let p2 = plan(&p1, &a);
        let p3 = plan(&p2, &a);

        assert_eq!("v<<A>>^A<A>AvA<^AA>Av<AAA>^A".len(), p2.len());
        assert_eq!(
            "v<A<AA>>^AvAA^<A>Av<<A>>^AvA^Av<<A>>^AAv<A>A^A<A>Av<A<A>>^AAA<Av>A^A".len(),
            p3.len()
        );
    }

    #[test]
    fn solve_mult() {
        let n = parse_map(NUMPAD);
        let a = parse_map(ARROWS);

        for (s, res) in [
            (
                "029A",
                "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A",
            ),
            (
                "980A",
                "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A",
            ),
            (
                "179A",
                "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A",
            ),
            (
                "456A",
                "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A",
            ),
            (
                "379A",
                "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A",
            ),
        ] {
            let p1 = plan(s.as_bytes(), &n);
            let p2 = plan(&p1, &a);
            let p3 = plan(&p2, &a);

            assert_eq!(
                format!("{s}:{}", res.as_bytes().len()),
                format!("{s}:{}", p3.len())
            );
        }
    }

    #[test]
    fn solve_p1() {
        assert_eq!(
            126384,
            Day21::part1(
                "
                029A
                980A
                179A
                456A
                379A
                "
            )
        );
    }
}
