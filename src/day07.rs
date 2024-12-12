use core::ops::Add;
use std::ops::Mul;

use crate::Day;

pub struct Day7 {}

fn parse(input: &str) -> Vec<(i64, Vec<i64>)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (a, bs) = line.trim().split_once(':').expect("has a colon");
            let bs = bs.split_whitespace();

            (
                a.parse().expect("is a number"),
                bs.map(|b| b.parse().expect("is a number")).collect(),
            )
        })
        .collect()
}

fn add<T: Add>(a: T, b: T) -> <T as Add>::Output {
    a + b
}

fn mul<T: Mul>(a: T, b: T) -> <T as Mul>::Output {
    a * b
}

fn concat(a: i64, b: i64) -> i64 {
    // Here's the naive version as formulated
    // format!("{a}{b}").parse().expect("is a number")

    // but the above takes 0.4s on the final input, while
    // the below takes 0.05s e.g. an 8x speedup.
    let mut mag = 10;
    while b >= mag {
        mag *= 10;
    }

    a * mag + b
}

fn count_solutions(answer: i64, operands: &[i64], operators: &[fn(i64, i64) -> i64]) -> usize {
    operands
        .iter()
        .fold(vec![], |poss, &operand| {
            let mut new_poss = vec![];
            if poss.is_empty() {
                new_poss.push(operand)
            } else {
                for operator in operators {
                    // the trick is to keep our possibilities as pruned as possible
                    // in order to limit the number of iters in this triple-nested loop
                    for v in &poss {
                        let newval = operator(*v, operand);
                        if newval <= answer {
                            new_poss.push(newval);
                        }
                    }
                }
            }

            new_poss
        })
        .into_iter()
        .filter(|v| *v == answer)
        .count()
}

impl Day<i64> for Day7 {
    fn part1(input: &str) -> i64 {
        let mut sum = 0;

        for (answer, operands) in parse(input) {
            if count_solutions(answer, &operands, &[add::<i64>, mul::<i64>]) > 0 {
                sum += answer
            }
        }

        sum
    }

    fn part2(input: &str) -> i64 {
        // let mut sum = 0;

        // for (answer, operands) in parse(input) {
        //     if count_solutions(answer, &operands, &[add::<i64>, mul::<i64>, concat]) > 0 {
        //         sum += answer
        //     }
        // }

        // sum

        parse(input)
            .into_iter()
            .map(|(answer, operands)| {
                if count_solutions(answer, &operands, &[add::<i64>, mul::<i64>, concat]) > 0 {
                    answer
                } else {
                    0
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &'static str = "
        190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20
        ";

    #[test]
    fn test_input_part1() {
        let a = Day7::part1(TEST_INPUT);
        assert_eq!(a, 3749);
    }

    #[test]
    fn test_input_part2() {
        let a = Day7::part2(TEST_INPUT);
        assert_eq!(a, 11387);
    }
}
