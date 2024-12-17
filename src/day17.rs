use crate::Day;

#[derive(Clone, Debug)]
struct Machine {
    a: u64,
    b: u64,
    c: u64,

    ip: usize,
    program: Vec<u64>,
}

fn parse(input: &str) -> Machine {
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut p = vec![];

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let (left, right) = line.split_once(": ").unwrap();

        match left {
            "Register A" => {
                a = right.parse().unwrap();
            }
            "Register B" => {
                b = right.parse().unwrap();
            }
            "Register C" => {
                c = right.parse().unwrap();
            }
            "Program" => {
                p = right.split(",").map(|v| v.parse().unwrap()).collect();
            }
            _ => {
                println!("unhandled line {}", line);
            }
        }
    }

    Machine {
        a,
        b,
        c,
        program: p,
        ip: 0,
    }
}

#[inline(always)]
fn combo(m: &Machine, operand: u64) -> u64 {
    match operand {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => m.a,
        5 => m.b,
        6 => m.c,
        7 => panic!("reserved"),
        _ => panic!("unexpected operand {operand}"),
    }
}

fn run(m: Machine) -> Vec<u64> {
    let mut m = m;
    let mut outputs = Vec::<u64>::new();

    loop {
        match m.program[m.ip] {
            0 => {
                // adv
                let num = m.a;
                let denom = (2 as u64).pow(combo(&m, m.program[m.ip + 1]) as u32);
                m.a = num / denom;
                m.ip += 2;
            }
            1 => {
                // bxl
                let lit = m.program[m.ip + 1];
                m.b = m.b ^ lit;
                m.ip += 2;
            }
            2 => {
                // bst
                let op = combo(&m, m.program[m.ip + 1]);
                m.b = op & 0b111;
                m.ip += 2;
            }
            3 => {
                // jnz
                if m.a != 0 {
                    m.ip = m.program[m.ip + 1] as usize;
                } else {
                    m.ip += 2;
                }
            }
            4 => {
                // bxc
                m.b = m.b ^ m.c;
                m.ip += 2;
            }
            5 => {
                // out
                outputs.push(combo(&m, m.program[m.ip + 1]) & 0b111);
                m.ip += 2;
            }
            6 => {
                // bdv
                let num = m.a;
                let denom = (2 as u64).pow(combo(&m, m.program[m.ip + 1]) as u32);
                m.b = num / denom;
                m.ip += 2;
            }
            7 => {
                // cdv
                let num = m.a;
                let denom = (2 as u64).pow(combo(&m, m.program[m.ip + 1]) as u32);
                m.c = num / denom;
                m.ip += 2;
            }
            _ => {
                let opcode = m.program[m.ip];
                panic!("unhandled opcode {opcode}");
            }
        }

        if m.ip >= m.program.len() {
            break;
        }
    }

    outputs
}

fn recur(machine: &Machine, target: &[u64], current: u64) -> Option<u64> {
    for idx in 0..8 {
        let mut m = machine.clone();
        let nxt = current * 8 + idx;

        if nxt == 0 {
            continue;
        }

        m.a = nxt;
        let res = run(m);
        // println!("{:?} -> {:?} {:?}", nxt, res, target);

        // Success condition
        if res == target {
            return Some(nxt);
        }

        // overshoot - no point descending
        if res.len() > target.len() {
            return None;
        }

        // Check res against tail of target
        let rlen = res.len();
        let tlen = target.len();
        let tail = &target[(tlen - rlen)..];
        assert!(tail.len() <= res.len());

        // And don't bother recursing if we don't have a matching tail already
        if tail != res {
            continue;
        }

        // Otherwise recursing is all we have left!
        if let Some(v) = recur(machine, target, nxt) {
            return Some(v);
        }
    }

    None
}

pub struct Day17 {}

impl Day<String> for Day17 {
    fn part1(input: &str) -> String {
        run(parse(input))
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }

    fn part2(input: &str) -> String {
        let m = parse(input);
        let a_out = recur(&m, &m.program, 0).unwrap();
        a_out.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &'static str = "
        Register A: 729
        Register B: 0
        Register C: 0

        Program: 0,1,5,4,3,0
    ";

    #[test]
    fn t1() {
        assert_eq!(vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0], run(parse(&TEST_INPUT)));
    }

    const TEST_INPUT_2: &'static str = "
        Register A: 2024
        Register B: 0
        Register C: 0

        Program: 0,3,5,4,3,0
    ";

    #[test]
    fn t2() {
        assert_eq!("117440", Day17::part2(TEST_INPUT_2));
    }
}
