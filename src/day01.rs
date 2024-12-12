use std::{collections::HashMap, iter::zip};

use crate::Day;

pub struct Day1 {}

impl Day<i32> for Day1 {
    fn part1(input: &str) -> i32 {
        let (left, right) = parse(input);

        let mut sum = 0;
        for (a, b) in zip(left, right) {
            let diff = a - b;
            let mag = diff.abs();

            sum += mag;
        }

        sum
    }

    fn part2(input: &str) -> i32 {
        let (left, right) = parse(input);

        let freqs = right.iter().fold(HashMap::new(), |mut acc, v| {
            if let Some(freq) = acc.get_mut(v) {
                *freq += 1;
            } else {
                acc.insert(v, 1);
            }

            acc
        });

        left.iter().map(|v| v * freqs.get(v).unwrap_or(&0)).sum()
    }
}

fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut a = vec![];
    let mut b = vec![];

    input.trim().lines().for_each(|line| {
        let ns: Vec<i32> = line
            .trim()
            .split_ascii_whitespace()
            .map(|n| n.parse().expect("is a number"))
            .collect();

        a.push(ns[0]);
        b.push(ns[1]);
    });

    a.sort();
    b.sort();

    (a, b)
}

#[cfg(test)]
mod test {
    #[test]
    fn parse() {
        let test_in = "
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
        ";

        assert_eq!(
            super::parse(&test_in),
            (vec!(1, 2, 3, 3, 3, 4), vec!(3, 3, 3, 4, 5, 9))
        );
    }
}
