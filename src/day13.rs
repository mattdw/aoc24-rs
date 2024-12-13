use nalgebra::{Matrix2, Vector2};
use regex::{Captures, Regex};

fn button_pat() -> Regex {
    Regex::new(r"Button (A|B): X\+(\d+), Y\+(\d+)").expect("regex problem")
}

fn prize_pat() -> Regex {
    Regex::new(r"Prize: X=(\d+), Y=(\d+)").expect("prize regex broken")
}

use crate::Day;

#[derive(Debug, PartialEq)]
struct Machine {
    buttons: Matrix2<i64>,
    prize: Vector2<i64>,
}

fn get_num(a: &Captures, i: usize) -> i64 {
    a.get(i)
        .unwrap()
        .as_str()
        .parse()
        .expect("matches a number")
}

fn parse(input: &str) -> Vec<Machine> {
    let pat = button_pat();
    let prize_p = prize_pat();

    input
        .trim()
        .split("\n\n")
        .map(|section| {
            let lines: Vec<&str> = section.trim().lines().collect();

            let a = pat
                .captures(lines[0])
                .expect("first line should match button");
            let b = pat
                .captures(lines[1])
                .expect("second line should match button");
            let p = prize_p
                .captures(lines[2])
                .expect("third line should match prize");

            Machine {
                buttons: Matrix2::<i64>::new(
                    get_num(&a, 2),
                    get_num(&b, 2),
                    get_num(&a, 3),
                    get_num(&b, 3),
                ),
                prize: Vector2::new(get_num(&p, 1), get_num(&p, 2)),
            }
        })
        .collect()
}

fn solve_machine(machine: &Machine, offset: Vector2<i64>) -> Option<Vector2<i64>> {
    let prize = machine.prize + offset;
    let ax = machine.buttons[0];
    let bx = machine.buttons[2];
    let ay = machine.buttons[1];
    let by = machine.buttons[3];

    // I knew I needed an analytical solution, but had a devil of a time
    // finding it.
    //
    // It's the 'cross-multiplication method' for solving linear equations
    // of two variables.

    let na = (by * prize.x - bx * prize.y) / (by * ax - bx * ay);
    let nb = (ay * prize.x - ax * prize.y) / (ay * bx - ax * by);

    let x = na * ax + nb * bx;
    let y = na * ay + nb * by;

    if x == prize.x && y == prize.y {
        // println!("{:?} {:?}", na, nb);
        Some(Vector2::new(na, nb))
    } else {
        None
    }
}

pub struct Day13 {}

impl Day<i64> for Day13 {
    fn part1(input: &str) -> i64 {
        let machines = parse(input);

        let mut winnable: Vec<(Machine, Vector2<i64>)> = vec![];
        for machine in machines {
            match solve_machine(&machine, Vector2::zeros()) {
                Some(v) => {
                    winnable.push((machine, v));
                }
                None => {}
            }

            // This worked, but at about 50x runtime.
            //
            // for a in 0..=100 {
            //     for b in 0..=100 {
            //         let v = Vector2::new(a, b);
            //         let res = machine.buttons * v;
            //         if res == machine.prize {
            //             winnable.push((machine, v));
            //             println!("{:?}", v);
            //             continue 'machine;
            //         }
            //         if res.x > machine.prize.x && res.y > machine.prize.y {
            //             break;
            //         }
            //     }
            // }
        }

        winnable
            .iter()
            .map(|(_m, input)| input.x * 3 + input.y)
            .sum()
    }

    fn part2(input: &str) -> i64 {
        let machines = parse(input);
        let offset = Vector2::new(10000000000000, 10000000000000);

        let mut winnable: Vec<(Machine, Vector2<i64>)> = vec![];
        for machine in machines {
            match solve_machine(&machine, offset) {
                Some(v) => {
                    winnable.push((machine, v));
                }
                None => {}
            }
        }

        winnable
            .iter()
            .map(|(_m, input)| input.x * 3 + input.y)
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &'static str = "
    Button A: X+94, Y+34
    Button B: X+22, Y+67
    Prize: X=8400, Y=5400

    Button A: X+26, Y+66
    Button B: X+67, Y+21
    Prize: X=12748, Y=12176

    Button A: X+17, Y+86
    Button B: X+84, Y+37
    Prize: X=7870, Y=6450

    Button A: X+69, Y+23
    Button B: X+27, Y+71
    Prize: X=18641, Y=10279
    ";

    #[test]
    fn t_parse() {
        let machines = parse(TEST_INPUT);
        println!("{:?}", machines);
        assert_eq!(machines.len(), 4);
    }

    #[test]
    fn t_machine_1() {
        let ms = parse(TEST_INPUT);
        let m = &ms[0];

        // v represents (button_a_presses, button_b_presses)
        let v = Vector2::new(80, 40);
        // by multiplying by the buttons matrix
        let r = m.buttons * v;
        // we hopefully reach the prize location
        assert_eq!(r, m.prize);
    }

    #[test]
    fn t_part1() {
        assert_eq!(480, Day13::part1(TEST_INPUT));
    }

    #[test]
    fn t_part2() {
        assert_eq!(875318608908, Day13::part2(TEST_INPUT));
    }
}
