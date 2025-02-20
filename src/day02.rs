use crate::{iterext::IterExt, Day};

fn parse(input: &str) -> Vec<Vec<i8>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|v| v.parse().expect("should be a number"))
                .collect()
        })
        .collect()
}

fn report_is_safe(r: &[i8]) -> bool {
    let mut increasing: Option<bool> = None;
    for (a, b) in r.pairs() {
        let diff = a - b;
        let sign = diff.signum() < 0;

        match increasing {
            None => increasing = Some(sign),
            Some(v) => {
                if v != sign {
                    return false;
                }
            }
        }

        let step = diff.abs();
        if step == 0 || step > 3 {
            return false;
        }
    }

    true
}

// struct DroppingOne<'a> {
//     slice: &'a [i8],
//     curr: usize,
// }

// impl Iterator for DroppingOne<'_> {
//     type Item = Vec<i8>;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.curr >= self.slice.len() {
//             return None;
//         }

//         let mut v = Vec::with_capacity(self.slice.len() - 1);
//         v.extend_from_slice(&self.slice[0..self.curr]);
//         v.extend_from_slice(&self.slice[self.curr + 1..]);

//         self.curr += 1;

//         Some(v)
//     }
// }

// fn dropping_one(r: &[i8]) -> DroppingOne {
//     DroppingOne { slice: r, curr: 0 }
// }

fn dropping_one(r: &[i8]) -> impl Iterator<Item = Vec<i8>> + '_ {
    let mut curr = 0;
    let slice = r;

    std::iter::from_fn(move || {
        if curr >= slice.len() {
            return None;
        }

        let mut v = Vec::with_capacity(slice.len() - 1);
        v.extend_from_slice(&slice[0..curr]);
        v.extend_from_slice(&slice[curr + 1..]);

        curr += 1;

        Some(v)
    })
}

pub struct Day2 {}

impl Day<usize> for Day2 {
    fn part1(input: &str) -> usize {
        parse(input).iter().filter(|r| report_is_safe(r)).count()
    }

    fn part2(input: &str) -> usize {
        parse(input)
            .into_iter()
            .filter(|r| dropping_one(r).any(|r| report_is_safe(&r)))
            .count()
    }
}
