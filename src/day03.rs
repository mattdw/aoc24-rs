use crate::Day;

use regex::Regex;

fn mul_pat() -> Regex {
    Regex::new(r"mul\((\d+),(\d+)\)").expect("is incorrect?")
}

fn all_pat() -> Regex {
    Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").expect("is incorrect?")
}
pub struct Day3 {}

impl Day<i64> for Day3 {
    fn part1(input: &str) -> i64 {
        let pat = mul_pat();

        let matches = pat
            .captures_iter(input)
            .map(|m| {
                let nums: Vec<i64> = [1, 2]
                    .iter()
                    .map(|num| {
                        m.get(*num)
                            .unwrap()
                            .as_str()
                            .parse()
                            .expect("should be num")
                    })
                    .collect();
                (nums[0], nums[1])
            })
            .collect::<Vec<_>>();

        matches.iter().map(|(a, b)| a * b).sum()
    }

    fn part2(input: &str) -> i64 {
        let pat = all_pat();

        pat.captures_iter(input)
            .fold((0, true), |(sum, active), m| {
                let r = &m.get(0).unwrap().as_str()[0..3];
                match r {
                    "do(" => (sum, true),
                    "don" => (sum, false),
                    "mul" => {
                        if !active {
                            return (sum, active);
                        }
                        let nums: Vec<i64> = [1, 2]
                            .iter()
                            .map(|num| {
                                m.get(*num)
                                    .unwrap()
                                    .as_str()
                                    .parse()
                                    .expect("should be num")
                            })
                            .collect();
                        (sum + nums[0] * nums[1], true)
                    }
                    _ => unreachable!(),
                }
            })
            .0
    }
}
