use std::collections::HashMap;

use crate::Day;

pub struct Day11 {}

type Stone = u64;

#[inline(never)]
fn num_digits(s: Stone) -> (u32, bool) {
    // if s < 10 {
    //     return (1, false);
    // }

    // let mut s = s;
    // let mag = 100;
    // let mut digits = 2;
    // while s >= mag {
    //     s /= mag;
    //     digits += 2;
    //     if s < 10 {
    //         return (digits - 1, false);
    //     }
    // }

    // (digits, true)

    if s == 0 {
        (1, false)
    } else {
        let digits = s.ilog10() + 1;
        (digits, digits % 2 == 0)
    }
}

#[inline(never)]
fn halves(s: Stone, num_digits: u32) -> (Stone, Stone) {
    let mag = (10 as Stone).pow(num_digits / 2);
    // assert_eq!(num_digits, s.to_string().len() as u32);

    (s / mag, s % mag)
}

#[allow(dead_code)]
fn step(v: &[Stone]) -> Vec<Stone> {
    let mut v2 = Vec::with_capacity(v.len());
    for &s in v {
        if s == 0 {
            v2.push(1);
            continue;
        }

        let (count, is_even) = num_digits(s);
        if is_even {
            let (left, right) = halves(s, count);
            v2.push(left);
            v2.push(right);
            continue;
        }
        v2.push(s * 2024);
    }
    v2
}

fn count_splits(s: Stone, depth: u64, cache: &mut HashMap<(Stone, u64), u64>) -> u64 {
    if depth == 0 {
        return 1;
    }

    if let Some(&v) = cache.get(&(s, depth)) {
        return v;
    }

    if s == 0 {
        let out = count_splits(1, depth - 1, cache);
        cache.insert((s, depth), out);
        return out;
    }
    let (digits, is_even) = num_digits(s);
    if is_even {
        let (left, right) = halves(s, digits);
        let out = count_splits(left, depth - 1, cache) + count_splits(right, depth - 1, cache);
        cache.insert((s, depth), out);
        return out;
    }

    let out = count_splits(s * 2024, depth - 1, cache);
    cache.insert((s, depth), out);

    out
}

fn parse(input: &str) -> Vec<Stone> {
    input
        .split_whitespace()
        .map(|n| {
            n.parse::<Stone>()
                .expect("data should only contain numbers")
        })
        .collect()
}

impl Day<u64> for Day11 {
    fn part1(input: &str) -> u64 {
        let v = parse(input);
        let mut cache = HashMap::new();
        let sum: u64 = v
            .into_iter()
            .map(|val| count_splits(val, 25, &mut cache))
            .sum();
        // v.len() as u64
        // println!("{} vals cached", cache.len());
        sum
    }

    fn part2(input: &str) -> u64 {
        let v = parse(input);
        let mut cache = HashMap::new();
        let sum: u64 = v
            .into_iter()
            .map(|val| count_splits(val, 75, &mut cache))
            .sum();
        // v.len() as u64
        // println!("{} vals cached", cache.len());
        sum
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn num_digits_t() {
        assert_eq!((5, false), num_digits(12345));
        assert_eq!((1, false), num_digits(0));
        assert_eq!((4, true), num_digits(9876));
        assert_eq!((6, true), num_digits(987600));
        assert_eq!((9, false), num_digits(123456789));
    }

    #[test]
    fn halves_t() {
        assert_eq!((10, 0), halves(1000, 4));
        assert_eq!((5, 4), halves(54, 2));
        assert_eq!((1234, 5678), halves(12345678, 8));
    }

    const TEST_INPUT: &[Stone] = &[0, 1, 10, 99, 999];
    const TEST_INPUT_2: &[Stone] = &[125, 17];

    #[test]
    fn step_once() {
        assert_eq!(vec![1, 2024, 1, 0, 9, 9, 2021976], step(TEST_INPUT))
    }

    #[test]
    fn step_6() {
        let mut v = Vec::from(TEST_INPUT_2);
        for _i in 0..6 {
            v = step(&v);
        }

        assert_eq!(
            vec![
                2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6,
                0, 3, 2
            ],
            v
        );

        assert_eq!(22, v.len());
    }
}
